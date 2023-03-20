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
        def wrapper(*args, **kwargs):
            runs = []
            for _ in range(times):
                runs.append(func(*args, **kwargs))
            return runs
        return wrapper
    return decorator


@run_multiple_times(times=1)
def run_simulation(input_data: str):
    simulation_process = subprocess.Popen(["make", "-s", "run-raw", "BIN=simulation", "USE_DOCKER=FALSE",
                                           f'RUN_ARGS=-i /dev/stdin -o /dev/stdout --max-duration 10'], stdout=subprocess.PIPE, stdin=subprocess.PIPE, text=True)

    simulation_process.stdin.write(input_data)
    simulation_process.stdin.close()

    analyzer_process = subprocess.Popen(["make", "-s", "run-raw", "BIN=frame_analyzer", "USE_DOCKER=FALSE",
                                        f'RUN_ARGS=-i /dev/stdin -o /dev/fd/{simulation_process.stdout.fileno()} -a /dev/stdout'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, pass_fds=(simulation_process.stdout.fileno(),))

    analysis, _ = analyzer_process.communicate(input_data)

    return pd.read_csv(
        StringIO(analysis),
        header=None,
        names=['t', 'va'],
        index_col='t',
        dtype={'t': np.float64, 'va': np.float64}
    )


def q2():
    L = 20
    Rc = 1
    r = .25

    df = pd.DataFrame({
        'N': pd.Series(dtype=int),
        'M': pd.Series(dtype=int),
        'cyclic': pd.Series(dtype=bool),
        'brute_force': pd.Series(dtype=bool),
        'time': pd.Series(dtype=float)
    })

    for N in [100, 200, 400, 800, 1600]:
        for noise in [0, 0.1, 0.2, 0.5, 1]:
            input_data = generate(N, 7.0, 0.5, noise, 0.03, None)
            run_times = run_simulation(input_data)
            for run_time in run_times:
                df = pd.concat(
                    [df, pd.DataFrame([{'N': N, 'noise': noise, 'time': run_time}], columns=df.columns)], ignore_index=True)
            avg = sum(run_times) / len(run_times)
            print(f"N={N}, noise={noise}, times={avg}")

    df.to_pickle("data/simulation_runs.pkl")
    print(df)


def q3():
    L = 20
    Rc = 1
    r = .25

    for N in [100, 200, 400, 800, 1600]:
        avg_times_1 = []
        avg_times_2 = []
        print("=====================================")
        for M in range(1, L // Rc):
            input_data = generate(N, L, M, Rc, r_min=r, r_max=r, seed=0)
            avg_1 = run_simulation(input_data, cyclic=True, brute_force=False)
            avg_2 = run_simulation(input_data, cyclic=False, brute_force=False)
            avg_times_1.append([M, avg_1])
            avg_times_2.append([M, avg_2])
            print(f"N={N}, M={M}")
            print(f"cyclic={True}, brute_force={False}, average_time={avg_1}")
            print(f"cyclic={False}, brute_force={False}, average_time={avg_2}")
        print("-------------------------------------")
        print(
            f"Optimal M for cyclic={True}, brute_force={False}: (M, time)={min(avg_times_1, key=lambda x: x[1])}")
        print(
            f"Optimal M for cyclic={False}, brute_force={False}: (M, time)={min(avg_times_2, key=lambda x: x[1])}")
        print("=====================================")


if __name__ == "__main__":
    print(run_simulation(generate(300, 7.0, 0.5, 0.2, 0.03)))
