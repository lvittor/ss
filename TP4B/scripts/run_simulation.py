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

        data = diff_analyze(os.fdopen(create_reader(sim1)), os.fdopen(create_reader(sim2)), input_data)
        data['k'] = k1
        df = pd.concat([df, data], ignore_index=False)

    df.to_csv("data/simulation_runs_ks.csv", index=False, na_rep='NaN')
    print(df)


if __name__ == "__main__":
    run_multiple_ks()
