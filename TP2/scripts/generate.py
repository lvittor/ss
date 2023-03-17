import random
from typing import Optional
import math

def generate(n: int, l: float, rc: float, noise: float, speed: float, runtime_seed: Optional[int] = None, seed: Optional[int] = None) -> str:
    if seed is not None:
        random.seed(seed)
    noise_seed = 'any' if runtime_seed is None else str(runtime_seed)
    out = '\n'.join(map(str, [noise_seed, n, l, rc, noise])) + '\n'

    for i in range(n):
        angle = random.uniform(-math.pi, math.pi)
        out += f'{i} ' + ' '.join(map("{:f}".format, [random.uniform(0, l),
                        random.uniform(0, l), math.cos(angle) * speed, math.sin(angle) * speed])) + '\n'

    return out


if __name__ == "__main__":
    print(generate(300, 7.0, 1.0, 2.0, .03), end='')
