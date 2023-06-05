import random
from typing import Optional
import math


def generate(n: int, room_side: float,
             max_speed: float,
             min_radius: float,
             max_radius: float,
             exit_size: float,
             far_exit_distance: float,
             far_exit_size: float,
             runtime_seed: Optional[int] = None,
             seed: Optional[int] = None) -> str:
    if seed is not None:
        random.seed(seed)
    noise_seed = 'any' if runtime_seed is None else str(runtime_seed)
    out = '\n'.join(map(str, [
                    noise_seed,
                    n,
                    room_side,
                    max_speed,
                    min_radius,
                    max_radius,
                    exit_size,
                    far_exit_distance,
                    far_exit_size])) + '\n'

    for i in range(n):
        out += f'{i} ' + ' '.join(map("{:f}".format, [random.uniform(0, room_side),
                                                      random.uniform(0, room_side)])) + '\n'

    return out


if __name__ == "__main__":
    print(generate(
        n=200,
        room_side=20,
        exit_size=1.2,
        far_exit_distance=10,
        far_exit_size=3,
        max_speed=2,
        min_radius=0.1,
        max_radius=0.32,
        runtime_seed=1234,
    ), end='')
