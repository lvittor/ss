import random
from typing import Optional
import math


def generate(table_width: float, table_height: float, min_white_y: float, hole_diameter: float, ball_diameter: float, ball_mass: float, balls_noise: float = 0, seed: Optional[int] = None) -> str:
    if seed is not None:
        random.seed(seed)
    out: str = ""

    out += f"{table_width}\n{table_height}\n{hole_diameter}\n{ball_diameter}\n{ball_mass}\n{16}\n"

    ball_id: int = 0

    def add_ball(x: float, y: float, vx: float = 0, vy: float = 0):
        nonlocal ball_id, out
        out += f"{ball_id} {x} {y} {vx} {vy}\n"
        ball_id += 1

    white_y = random.uniform(min_white_y, table_height / 2)
    add_ball(table_width / 4, white_y, 200, 0)

    ball_separation = ball_diameter * 1.1
    triangle_height = math.sqrt(
        ball_separation ** 2 - (ball_separation / 2) ** 2)

    for rank in range(5):
        x = table_width * 3 / 4 + rank * triangle_height
        for i in range(rank + 1):
            y = table_height / 2 + ((i - rank / 2) * ball_separation)
            add_ball(x + random.uniform(-balls_noise/2, balls_noise/2),
                     y + random.uniform(-balls_noise/2, balls_noise/2))
    return out


if __name__ == "__main__":
    print(generate(table_width=224, table_height=112, min_white_y=42,
          hole_diameter=5.7*2, ball_diameter=5.7, ball_mass=165, balls_noise=0.1), end='')
