from io import BytesIO, StringIO
from math import pi
from generate import generate
from typing import Optional, TextIO
import subprocess
import pandas as pd
import numpy as np

# Decorator for running the function run_cim multiple times and get the average


def run_multiple_times(times: int):
    def decorator(func):
        def wrapper(input_generator):
            df = pd.DataFrame()
            for i in range(times):
                run = func(input_generator())
                run['run'] = i
                df = pd.concat([df, run], ignore_index=True)
            return df
        return wrapper
    return decorator


@run_multiple_times(times=3)
def run_simulation(input_data: str):
    simulation_process = subprocess.Popen(["make", "-s", "run-raw", "BIN=simulation", "USE_DOCKER=FALSE",
                                           f'RUN_ARGS=-i /dev/stdin -o /dev/stdout --max-duration 1000'], stdout=subprocess.PIPE, stdin=subprocess.PIPE, text=True)

    simulation_process.stdin.write(input_data)
    simulation_process.stdin.close()

    analyzer_process = subprocess.Popen(["make", "-s", "run-raw", "BIN=frame_analyzer", "USE_DOCKER=FALSE",
                                        f'RUN_ARGS=-i /dev/stdin -o /dev/fd/{simulation_process.stdout.fileno()} -a /dev/stdout'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, pass_fds=(simulation_process.stdout.fileno(),))

    analysis, _ = analyzer_process.communicate(input_data)

    return pd.read_csv(
        StringIO(analysis),
        header=None,
        names=['t', 'va'],
        dtype={'t': np.float64, 'va': np.float64}
    )


def run():
    L = 20
    Rc = 0.5
    speed = 0.03

    df = pd.DataFrame({
        'N': pd.Series(dtype=int),
        'noise': pd.Series(dtype=float),
        'run': pd.Series(dtype=int),
        't': pd.Series(dtype=float),
        'va': pd.Series(dtype=float),
    })

    for N in [40, 100, 400, 4000, 10000]:
        for noise in [1]:
            print(f"N={N}, noise={noise}")
            data = run_simulation(lambda :generate(N, L, Rc, noise, speed, None))
            data['N'] = N
            data['noise'] = noise
            df = pd.concat([df, data], ignore_index = False)

    df.to_pickle("data/simulation_runs_b.pkl")
    print(df)

if __name__ == "__main__":
    # print(run_simulation(generate(300, 7.0, 0.5, 0.2, 0.03)))
    run()
