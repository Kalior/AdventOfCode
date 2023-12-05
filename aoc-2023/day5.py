from pathlib import Path
import numpy as np
from dataclasses import dataclass

from more_itertools import ichunked


@dataclass
class Map:
    name: str
    ranges: list[tuple[int, int, int]]

    @staticmethod
    def parse_line(line: str) -> tuple[int, int, int]:
        return tuple(int(v) for v in line.split())

    @staticmethod
    def parse_map(group: str) -> "Map":
        lines = group.splitlines()
        name = lines[0].replace(" map:", "")

        ranges = [Map.parse_line(line) for line in lines[1:]]

        return Map(name, ranges)

    def convert(self, v: int) -> int:
        for destination_start, source_start, length in self.ranges:
            if source_start <= v <= source_start + length:
                return destination_start + np.abs(v - source_start)

        return v


def parse_input() -> tuple[list[str], list[Map]]:
    p = Path("inputs") / "day5"
    with p.open("r") as f:
        input = f.read()

    input_groups = input.split("\n\n")

    seeds = [int(v) for v in input_groups[0].replace("seeds: ", "").split()]

    maps = input_groups[1:]

    maps = [Map.parse_map(map_) for map_ in maps]

    return seeds, maps


def _convert(v: int, maps: list[Map]):
    for map_ in maps:
        v = map_.convert(v)
    return v


def solve(seeds_and_maps: tuple[list[str], list[Map]]):
    seeds, maps = seeds_and_maps

    return min(_convert(seed, maps) for seed in seeds)


def solve_2(seeds_and_maps: tuple[list[str], list[Map]]):
    seeds, maps = seeds_and_maps

    range_seeds = [
        (seed_start, seed_start + length) for seed_start, length in ichunked(seeds, 2)
    ]

    rng = np.random.default_rng()
    some_seeds = np.array(
        [v for start, end in range_seeds for v in rng.integers(start, end, 1000)]
    )

    min_v = np.inf
    old_min = 0

    same_min_for_i = 0
    while same_min_for_i < 10:
        # Apply conversion to subset of seeds
        converted = np.array([_convert(seed, maps) for seed in some_seeds])

        # Update smallest as-yet found values
        old_min = min_v
        min_v = min(list(converted) + [old_min])
        print(min_v)

        # Resample `some_seeds` from around where we found the smallest values
        min_idx = converted.argsort()[:5]
        some_seeds = np.array(
            [
                v
                for m in min_idx
                for v in rng.integers(
                    some_seeds[m] - 10000, some_seeds[m] + 10000, 1000
                )
                if any(s <= v < e for s, e in range_seeds)
            ]
        )

        # Ensure we don't quit looking too early
        if min_v == old_min:
            same_min_for_i += 1
        else:
            same_min_for_i = 0

    return min_v


if __name__ == "__main__":
    games = parse_input()
    print(solve(games))
    print(solve_2(games))
