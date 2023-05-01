import random
from typing import Optional
import math


def rand_inside_circle(radius: float):
    r = radius * math.sqrt(random.random())
    theta = random.random() * 2 * math.pi

    x = r * math.cos(theta)
    y = r * math.sin(theta)

    return x, y


def generate(
    table_width: float,
    table_height: float,
    white_y: float,
    hole_diameter: float,
    ball_diameter: float,
    ball_mass: float,
    seed: Optional[int] = None,
    speed: int = 200,
) -> str:
    if seed is not None:
        random.seed(seed)
    out: str = ""

    out += f"{table_width}\n{table_height}\n{hole_diameter}\n{ball_diameter}\n{ball_mass}\n{16}\n"

    ball_id: int = 0

    def add_ball(x: float, y: float, vx: float = 0, vy: float = 0):
        nonlocal ball_id, out
        out += f"{ball_id} {x} {y} {vx} {vy}\n"
        ball_id += 1

    add_ball(table_width / 4, white_y, speed, 0)

    min_separation = 0.02
    max_separation = 0.03

    initial_separation = ball_diameter + max_separation / 2 + min_separation / 2
    max_random_module = (max_separation - min_separation) / 4

    triangle_height = math.sqrt(initial_separation**2 - (initial_separation / 2) ** 2)

    for rank in range(5):
        x = table_width * 3 / 4 + rank * triangle_height
        for i in range(rank + 1):
            y = table_height / 2 + ((i - rank / 2) * initial_separation)
            rx, ry = rand_inside_circle(max_random_module)
            add_ball(x + rx, y + ry)

    return out


if __name__ == "__main__":
    print(
        generate(
            table_width=224,
            table_height=112,
            white_y=random.uniform(42, 56),
            hole_diameter=5.7 * 2,
            ball_diameter=5.7,
            ball_mass=165,
        ),
        end="",
    )
