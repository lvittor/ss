import random
from typing import Optional
import math


def generate(n: int, l: float, rc: float, noise: float, speed: float, runtime_seed: Optional[int] = None, seed: Optional[int] = None) -> str:
    if seed is not None:
        random.seed(seed)
    raise NotImplementedError()
    out = ""

    return out


if __name__ == "__main__":
    print(generate(300, 7.0, 0.2, 0.5, .03, runtime_seed=1234), end='')
