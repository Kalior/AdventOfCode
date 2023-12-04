from pathlib import Path
import numpy as np


def parse_input() -> list[tuple[set[int], set[int]]]:
    p = Path("inputs") / "day4"
    with p.open("r") as f:
        input = f.read()

    def parse_group(group: str) -> list[int]:
        return [int(v) for v in group.split()]

    def parse_groups(groups: str) -> tuple[set[int], set[int]]:
        return tuple([set(parse_group(g)) for g in groups.split(" | ")])

    return [parse_groups(line.split(": ")[1]) for line in input.splitlines()]


def solve(scratchcards: list[tuple[set[int], set[int]]]):
    points = 0
    for winning, mine in scratchcards:
        same = winning.intersection(mine)
        if len(same) > 0:
            points += 2 ** (len(same) - 1)

    return points


def solve_2(scratchcards: list[tuple[set[int], set[int]]]):
    weights = np.ones(len(scratchcards), dtype=int)
    for i, (winning, mine) in enumerate(scratchcards):
        n_same = len(winning.intersection(mine))
        weights[i + 1 : i + n_same + 1] += weights[i]

    return weights.sum()


if __name__ == "__main__":
    games = parse_input()
    print(solve(games))
    print(solve_2(games))
