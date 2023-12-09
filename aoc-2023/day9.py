from pathlib import Path

import numpy as np


def parse_input() -> list[np.ndarray]:
    p = Path("inputs") / "day9"
    with p.open("r") as f:
        input = f.read()

    return [
        np.array([int(v) for v in line.split()], dtype=int)
        for line in input.splitlines()
    ]


def extrapolate(report: np.ndarray) -> int:
    extrapolations = [report]
    while not all(extrapolations[-1] == 0):
        extrapolations.append(np.diff(extrapolations[-1]))

    # Add zeros to the beginning and end of 0-valued array
    extrapolations[-1] = np.insert(extrapolations[-1], [0, -1], [0, 0])

    # Extrapolate forwards
    for i in range(len(extrapolations) - 2, -1, -1):
        new_v = extrapolations[i][-1] + extrapolations[i + 1][-1]
        extrapolations[i] = np.append(extrapolations[i], new_v)

    # Extrapolate backwards
    for i in range(len(extrapolations) - 2, -1, -1):
        new_v = extrapolations[i][0] - extrapolations[i + 1][0]
        extrapolations[i] = np.insert(extrapolations[i], 0, new_v)

    return extrapolations[0]


def solve(oasis_report: list[np.ndarray]):
    return sum(extrapolate(report)[-1] for report in oasis_report)


def solve_2(oasis_report: list[np.ndarray]):
    return sum(extrapolate(report)[0] for report in oasis_report)


if __name__ == "__main__":
    oasis_report = parse_input()
    print(f"Part one: {solve(oasis_report)}")
    print(f"Part two: {solve_2(oasis_report)}")
