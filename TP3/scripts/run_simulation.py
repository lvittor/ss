from io import BytesIO, StringIO
from math import pi
from multiprocessing.pool import ThreadPool
from generate import generate
from typing import Optional, TextIO
import subprocess
import pandas as pd
import numpy as np
from multiprocessing import Pool

# Decorator for running the function run_cim multiple times and get the average


def run_multiple_times(times: int):
    def decorator(func):
        def wrapper(input_generator):
            df = pd.DataFrame()
            with ThreadPool() as pool:
                for i, run in enumerate(
                    pool.imap(func, (input_generator() for _ in range(times)))
                ):
                    run["run"] = i
                    df = pd.concat([df, run], ignore_index=True)
            return df

        return wrapper

    return decorator


@run_multiple_times(times=1000)
def run_simulation(input_data: str):
    simulation_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=simulation",
            "USE_DOCKER=FALSE",
            f"RUN_ARGS=-i /dev/stdin -o /dev/stdout --max-duration 1000",
        ],
        stdout=subprocess.PIPE,
        stdin=subprocess.PIPE,
        text=True,
    )

    simulation_process.stdin.write(input_data)
    simulation_process.stdin.close()

    analyzer_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=analyze",
            "USE_DOCKER=FALSE",
            f"RUN_ARGS=-i /dev/stdin -o /dev/fd/{simulation_process.stdout.fileno()} -a /dev/stdout",
        ],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
        pass_fds=(simulation_process.stdout.fileno(),),
    )

    analysis, _ = analyzer_process.communicate(input_data)

    return pd.read_csv(
        StringIO(analysis),
        header=None,
        names=["t", "ball_count", "kinetic_energy"],
        dtype={"t": np.float64, "ball_count": np.uint64, "kinetic_energy": np.float64},
    )


def run():
    L = 20
    Rc = 0.5
    speed = 0.03

    df = pd.DataFrame(
        {
            "white_y": pd.Series(dtype=float),
            "run": pd.Series(dtype=int),
            "t": pd.Series(dtype=float),
            "ball_count": pd.Series(dtype=int),
            "kinetic_energy": pd.Series(dtype=float),
        }
    )

    for speed in range(200, 400, 800, 1600, 3200):
        print(f"white_y={speed}")
        data = run_simulation(
            lambda: generate(
                table_width=224,
                table_height=112,
                white_y=56,
                hole_diameter=5.7 * 2,
                ball_diameter=5.7,
                ball_mass=165,
                speed=speed,
            )
        )
        data["white_y"] = 56
        df = pd.concat([df, data], ignore_index=False)

    df.to_pickle("data/simulation_runs.pkl")
    print(df)


if __name__ == "__main__":
    # print(run_simulation(generate(300, 7.0, 0.5, 0.2, 0.03)))
    run()
