from math import pi
from generate import generate
from typing import Optional
import subprocess

def write_file(n: int, l: float, m: int, rc: float, seed: Optional[int] = None) -> None:
    file_string = generate(n, l, m, rc, seed)
    with open("/tmp/temp.txt", "w") as f:
        f.write(file_string)


# Decorator for running the function run_cim multiple times and get the average
def run_multiple_times(times: int):
    def decorator(func):
        def wrapper(*args, **kwargs):
            total = 0
            for _ in range(times):
                total += func(*args, **kwargs)
            average_time = total / times
            return average_time
        return wrapper
    return decorator
    

@run_multiple_times(times=10)
def run_cim(input_data: str, cyclic: bool = False, brute_force: bool = False):
    args = ""

    if cyclic:
        args += "--cyclic "
    if brute_force:
        args += "--brute-force "
    
    process = subprocess.Popen(["make", "-s", "-C", "cim-implementation",
                                "run-impl", f"INPUT_FILE_PATH=/dev/stdin", f"ARGS={args}"], stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, stdin=subprocess.PIPE)
    _, stderr = process.communicate(input=input_data.encode())
    return float(stderr)


def q2():
    L = 20
    Rc = 1
    r = .25

    for N in [100, 200, 400, 800, 1600]:
        for M in [1, 2, 4, 8, 16]:
            input_data = generate(N, L, M, Rc, r_min=r, r_max=r, seed=0)
            print("=====================================")
            print(f"N={N}, M={M}")
            print(f"cyclic={True}, brute_force={True}, average_time={run_cim(input_data, cyclic=True, brute_force=True)}")
            print(f"cyclic={True}, brute_force={False}, average_time={run_cim(input_data, cyclic=True, brute_force=False)}")
            print(f"cyclic={False}, brute_force={True}, average_time={run_cim(input_data, cyclic=False, brute_force=True)}")
            print(f"cyclic={False}, brute_force={False}, average_time={run_cim(input_data, cyclic=False, brute_force=False)}")
            print("=====================================")


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
            avg_1 = run_cim(input_data, cyclic=True, brute_force=False)
            avg_2 = run_cim(input_data, cyclic=False, brute_force=False)
            avg_times_1.append([M, avg_1])
            avg_times_2.append([M, avg_2])
            print(f"N={N}, M={M}")
            print(f"cyclic={True}, brute_force={False}, average_time={avg_1}")
            print(f"cyclic={False}, brute_force={False}, average_time={avg_2}")
        print("-------------------------------------")
        print(f"Optimal M for cyclic={True}, brute_force={False}: (M, time)={min(avg_times_1, key=lambda x: x[1])}")
        print(f"Optimal M for cyclic={False}, brute_force={False}: (M, time)={min(avg_times_2, key=lambda x: x[1])}")
        print("=====================================")

            

if __name__ == "__main__":
    q2()
    # q3()