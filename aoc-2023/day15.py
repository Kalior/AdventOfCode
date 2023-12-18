from collections import defaultdict
from pathlib import Path

import numpy as np


def parse_input() -> list[str]:
    p = Path("inputs") / "day15"
    with p.open("r") as f:
        input = f.read()
    return input.splitlines()[0].split(",")


def hash(seq: str) -> int:
    v = 0
    for c in seq:
        v += ord(c)
        v *= 17
        v %= 256
    return v


def solve(initialisation_sequence: list[str]):
    return sum([hash(v) for v in initialisation_sequence])


def solve_2(initialisation_sequence: list[str]):
    boxes = defaultdict(lambda: defaultdict())
    for v in initialisation_sequence:
        if "-" in v:
            label = v.replace("-", "")
            box_i = hash(label)
            boxes[box_i].pop(label, None)
        if "=" in v:
            label, focal_length = v.split("=")
            box_i = hash(label)
            boxes[box_i][label] = int(focal_length)

    return sum(
        [
            (box_i + 1) * (i + 1) * focal_length
            for box_i, box in boxes.items()
            for i, focal_length in enumerate(box.values())
        ]
    )


if __name__ == "__main__":
    initialisation_sequence = parse_input()
    print(f"Part one: {solve(initialisation_sequence)}")
    print(f"Part two: {solve_2(initialisation_sequence)}")
