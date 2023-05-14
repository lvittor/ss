from io import BytesIO, FileIO, StringIO
from math import pi
from multiprocessing.pool import ThreadPool
from os import pipe
import os
import threading
from generate import generate
from typing import Optional, TextIO
import subprocess
import pandas as pd
import numpy as np
from multiprocessing import Pool
import json
import tqdm

# Decorator for running the function run_cim multiple times and get the average


def run_multiple_times(times: int):
    def decorator(func):
        def wrapper(input_generator):
            df = pd.DataFrame()
            with ThreadPool() as pool:
                for i, run in enumerate(
                    tqdm.tqdm(pool.imap_unordered(func, (input_generator()
                              for _ in range(times))), total=times)
                ):
                    run["run"] = i
                    df = pd.concat([df, run], ignore_index=True)
            return df

        return wrapper

    return decorator


def run_simulation(k: int, output_every: int):
    print(k, output_every)
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

    return simulation_process


@run_multiple_times(times=24)
def run_simulation_analysis(args: tuple[str, int, int]):
    (input_data, k, target_ball_amount) = args
    # output_every = 0.1 / 10**-k
    output_every = 100000000
    simulation_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=simulation",
            "USE_DOCKER=FALSE",
            "RUN_ARGS=" +
            "-i /dev/stdin " +
            "-o /dev/stdout " +
            f"--delta-time-n={k} " +
            f"--output-every={output_every} " +
            f"--max-duration={10000} " +
            "--with-holes " +
            f"--min-ball-amount={target_ball_amount + 1}",
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

    df = pd.read_csv(
        StringIO(analysis),
        header=None,
        names=["t", "ball_count", "kinetic_energy"],
        dtype={"t": np.float64, "ball_count": np.uint64,
               "kinetic_energy": np.float64},
    )

    df = df[df['ball_count'] == target_ball_amount]
    df = df.drop(['kinetic_energy'], axis=1)
    df = df.rename(
        columns={'ball_count': 'final_ball_amount', 't': 'final_time'})
    return df


def writer_thread(w, data: str):
    w = os.fdopen(w, 'w')
    w.write(data)
    w.flush()
    w.close()


def create_reader(data: str):
    r, w = pipe()
    threading.Thread(target=writer_thread, args=(w, data)).start()
    return r


def diff_analyze(simulation_1, simulation_2, input_data: str):
    analyzer_process = subprocess.Popen(
        [
            "make",
            "-s",
            "run-raw",
            "BIN=diff_analyze",
            "USE_DOCKER=FALSE",
            f"RUN_ARGS=--output1=/dev/fd/{simulation_1.fileno()} --output2=/dev/fd/{simulation_2.fileno()} -a=/dev/stdout",
        ],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
        pass_fds=(simulation_1.fileno(),
                  simulation_2.fileno(),),
    )

    analysis, _ = analyzer_process.communicate(input_data)

    return pd.read_csv(StringIO(analysis))


def run_multiple_ks():
    df = pd.DataFrame({
        'k': pd.Series(dtype=int),
        't': pd.Series(dtype=np.float64),
        'phi': pd.Series(dtype=np.float64),
    })

    input_data = generate(
        table_width=224,
        table_height=112,
        white_y=56,
        hole_diameter=5.7*2,
        ball_diameter=5.7,
        ball_mass=165,
        seed=153789
    )
    # simulations = [(k, run_simulation(k, int(0.1 / 10**-k))) for k in range(2, 6 + 1)]

    # outputs = []
    # for k, simulation in simulations:
    # print(k)
    # outputs.append((k, simulation.communicate(input_data)[0]))

    # json.dump(outputs, open('outputs.json', 'w'));
    outputs = json.load(open('outputs.json'))

    for (k1, sim1), (k2, sim2) in zip(outputs, outputs[1::]):
        print(f"k={k1} - {k2}")

        data = diff_analyze(os.fdopen(create_reader(sim1)),
                            os.fdopen(create_reader(sim2)), input_data)
        data['k'] = k1
        df = pd.concat([df, data], ignore_index=False)

    df.to_csv("data/simulation_runs_ks.csv", index=False, na_rep='NaN')
    print(df)


def run_multiple_ys():
    df = pd.DataFrame({
        'white_y': pd.Series(dtype=float),
        'run': pd.Series(dtype=int),
        'final_time': pd.Series(dtype=float),
        'final_ball_amount': pd.Series(dtype=np.uint64),
    })

    for target_ball_amount in (8, 0):
        for white_y in np.linspace(42, 56, 2):
            print(f"target_ball_amount={target_ball_amount} white_y={white_y}")
            data = run_simulation_analysis(lambda: (generate(
                table_width=224,
                table_height=112,
                white_y=white_y,
                hole_diameter=5.7*2,
                ball_diameter=5.7,
                ball_mass=165
            ), 4, target_ball_amount))
            data['white_y'] = white_y
            df = pd.concat([df, data], ignore_index=True)

    df.to_csv("data/simulation_runs_ys.csv", index=False)
    print(df)


if __name__ == "__main__":
    # run_multiple_ks()
    run_multiple_ys()
