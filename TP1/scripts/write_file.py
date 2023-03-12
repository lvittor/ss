from generate import generate
from typing import Optional

def write_file(n: int, l: float, m: int, rc: float, seed: Optional[int] = None) -> None:
    file_string = generate(n, l, m, rc, seed)
    with open("/tmp/temp.txt", "w") as f:
        f.write(file_string)

if __name__ == "__main__":
    write_file(100, 100.0, 8, 10.0)