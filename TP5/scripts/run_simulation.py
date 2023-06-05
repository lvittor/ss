from io import BytesIO, StringIO
from math import pi
from multiprocessing.pool import ThreadPool

import tqdm
from generate import generate
from typing import Optional, TextIO
import subprocess
import pandas as pd
import numpy as np

# Decorator for running the function run_cim multiple times and get the average


def run_multiple_times():
    def decorator(func):
        def wrapper(input_generator, times: int):
            df = pd.DataFrame()
            with ThreadPool() as pool:
                for i, run in enumerate(
                    tqdm.tqdm(pool.imap_unordered(func, (input_generator()
                                                         for _ in range(times))), total=times)
                ):
                    run = func(input_generator())
                    run['run'] = i
                    df = pd.concat([df, run], ignore_index=True)
            return df
        return wrapper
    return decorator


@run_multiple_times()
def run_simulation(input_data: str):
    simulation_process = subprocess.Popen(["make", "-s", "run-raw", "BIN=simulation", "USE_DOCKER=FALSE",
                                           f'RUN_ARGS=--input /dev/stdin --output-exit-times=/dev/stdout --output-particles=/dev/null --outputs-per-second=1'],
                                          stdout=subprocess.PIPE, stdin=subprocess.PIPE, text=True)

    data, _ = simulation_process.communicate(input_data)
    data = list(map(float, data.splitlines()))

    return pd.DataFrame(enumerate(data), columns=['exit_n', 'time'])


def run1():
    df = run_simulation(lambda: generate(
        n=200,
        room_side=20,
        exit_size=1.2,
        far_exit_distance=10,
        far_exit_size=3,
        max_speed=2,
        min_radius=0.1,
        max_radius=0.32,
    ), 1000)

    df.to_csv("data/simulation_a.csv", index=False)
    print(df)


def run2():
    df = pd.DataFrame({
        'N': pd.Series(dtype=int),
        'd': pd.Series(dtype=float),
        'run': pd.Series(dtype=int),
        'exit_n': pd.Series(dtype=int),
        'time': pd.Series(dtype=float),
    })

    for N, d in zip([200, 260, 320, 380], [1.2, 1.8, 2.4, 3.0]):
        print(f"N={N}, d={d}")
        data = run_simulation(lambda: generate(
            n=N,
            room_side=20,
            exit_size=d,
            far_exit_distance=10,
            far_exit_size=3,
            max_speed=2,
            min_radius=0.1,
            max_radius=0.32,
        ), 100)
        data['N'] = N
        data['d'] = d
        df = pd.concat([df, data], ignore_index=False)

    df.to_csv("data/simulation_b.csv", index=False)
    print(df)


if __name__ == "__main__":
    # run1()
    run2()
