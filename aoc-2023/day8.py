from pathlib import Path
import itertools

import math
import numpy as np


def parse_input():
    p = Path("inputs") / "day8"
    with p.open("r") as f:
        input = f.read()

    lines = input.splitlines()
    moves = [0 if c == "L" else 1 for c in lines[0]]

    def _parse_line(line):
        line = line.replace("(", "").replace(")", "")
        key, vals = line.split(" = ")
        left, right = vals.split(", ")
        return key, np.array(list([left, right]))

    map_ = dict([_parse_line(line) for line in lines[2:]])

    return map_, moves


def solve(map_, moves):
    moves = itertools.cycle(moves)
    current_pos = "AAA"
    for i, move in enumerate(moves):
        current_pos = map_[current_pos][move]
        if current_pos == "ZZZ":
            return i + 1


def solve_2(map_, moves):
    moves = itertools.cycle(moves)
    current_poses = [key for key in map_.keys() if key[-1] == "A"]
    found_z_at = [0 for _ in current_poses]

    for i, move in enumerate(moves):
        current_poses = [map_[current_pos][move] for current_pos in current_poses]

        for j, p in enumerate(current_poses):
            if p[-1] == 'Z':
                found_z_at[j] = i + 1

        if all(found_z_at):
            break

    return math.lcm(*found_z_at)


if __name__ == "__main__":
    map_, moves = parse_input()
    print(f"Part one: {solve(map_, moves)}")
    print(f"Part two: {solve_2(map_, moves)}")
