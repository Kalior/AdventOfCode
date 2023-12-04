from pathlib import Path
from itertools import count
from dataclasses import dataclass, field
from typing import Iterable
from math import prod

from more_itertools import split_at


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
        print(2 ** (len(same) - 1))
        if len(same) > 0:
            points += 2 ** (len(same) - 1)

    return points


def solve_2(schema: list[tuple[set[int], set[int]]]):
    pass


if __name__ == "__main__":
    games = parse_input()
    print(solve(games))
    print(solve_2(games))
