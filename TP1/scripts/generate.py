import random
from typing import Optional


def generate(n: int, l: float, m: int, rc: float, r_min: float, r_max: float, seed: Optional[int] = None) -> str:
    if seed is not None:
        random.seed(seed)
    out = '\n'.join(map(str, [n, l, m, rc])) + '\n'

    for i in range(n):
        out += ' '.join(map(str, [i, random.uniform(0, l),
                        random.uniform(0, l), random.uniform(r_min, r_max)])) + '\n'

    return out


if __name__ == "__main__":
    print(generate(100, 100.0, 8, 10.0, .5, 3.0))
