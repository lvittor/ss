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

def run_simulation(input_data: str, k: int, output_every: int):
    simulation_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=simulation",
            "USE_DOCKER=FALSE",
            f"RUN_ARGS=-i /dev/stdin -o /dev/stdout --delta-time-n={k} --max-duration=100 --output-every={output_every}",
        ],
        stdout=subprocess.PIPE,
        stdin=subprocess.PIPE,
        text=True,
    )

    simulation_process.stdin.write(input_data)
    simulation_process.stdin.close()

    return simulation_process

def run_simulation_diff(input_data: str, k: int):
    steps = int(1e-1 // 10**-k)
    simulation_k = run_simulation(input_data, k, steps)
    simulation_k_plus = run_simulation(input_data, k + 1, steps * 10)

    analyzer_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=diff_analyze",
            "USE_DOCKER=FALSE",
            f"RUN_ARGS=--output1=/dev/fd/{simulation_k.stdout.fileno()} --output2=/dev/fd/{simulation_k_plus.stdout.fileno()} -a=/dev/stdout",
        ],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
        pass_fds=(simulation_k.stdout.fileno(),simulation_k_plus.stdout.fileno(),),
    )

    analysis, _ = analyzer_process.communicate(input_data)

    return pd.read_csv(StringIO(analysis))

def run_multiple_ks():
    df = pd.DataFrame({
        'k': pd.Series(dtype=int),
        't': pd.Series(dtype=np.float64),
        'phi': pd.Series(dtype=np.float64),
    })

    for k in range(2, 2+1):
        print(f"k={k}")
        data = run_simulation_diff(generate(
            table_width=224,
            table_height=112,
            white_y=56,
            hole_diameter=5.7*2,
            ball_diameter=5.7,
            ball_mass=165
        ), k)
        data['k'] = k
        df = pd.concat([df, data], ignore_index=False)

    df.to_csv("data/simulation_runs_ks.csv", index=False, na_rep='NaN')
    print(df.to_string())

if __name__ == "__main__":
    run_multiple_ks()
