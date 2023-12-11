from pathlib import Path
from enum import Enum, auto
import numpy as np
from collections import defaultdict


class Tile(Enum):
    NorthSouth = auto()
    Horizontal = auto()
    EastWest = auto()
    NorthEast = auto()
    NorthWest = auto()
    SouthWest = auto()
    SouthEast = auto()
    Ground = auto()
    Start = auto()

    @staticmethod
    def parse(v: str) -> "Tile":
        match v:
            case "|":
                return Tile.NorthSouth
            case "-":
                return Tile.EastWest
            case "L":
                return Tile.NorthEast
            case "J":
                return Tile.NorthWest
            case "7":
                return Tile.SouthWest
            case "F":
                return Tile.SouthEast
            case ".":
                return Tile.Ground
            case "S":
                return Tile.Start

    def blocks_south_north(self) -> bool:
        match self:
            case Tile.EastWest:
                return True
            case _:
                return False

    def blocks_east_south(self) -> bool:
        match self:
            case Tile.NorthSouth:
                return True
            case _:
                return False

    def goes_south(self) -> bool:
        match self:
            case Tile.SouthWest | Tile.SouthEast | Tile.NorthSouth | Tile.Start:
                return True
            case _:
                return False

    def goes_north(self) -> bool:
        match self:
            case Tile.NorthWest | Tile.NorthEast | Tile.NorthSouth | Tile.Start:
                return True
            case _:
                return False

    def goes_west(self) -> bool:
        match self:
            case Tile.EastWest | Tile.NorthWest | Tile.SouthWest | Tile.Start:
                return True
            case _:
                return False

    def goes_east(self) -> bool:
        match self:
            case Tile.EastWest | Tile.NorthEast | Tile.SouthEast | Tile.Start:
                return True
            case _:
                return False


def connects_to(
    pipes: dict[tuple[int, int], Tile],
    point: tuple[int, int],
    point_to: tuple[int, int],
):
    if (
        point[0] < point_to[0]
        and point[1] == point_to[1]
        and pipes[point].goes_south()
        and pipes[point_to].goes_north()
    ):
        return True
    elif (
        point[0] > point_to[0]
        and point[1] == point_to[1]
        and pipes[point].goes_north()
        and pipes[point_to].goes_south()
    ):
        return True
    elif (
        point[0] == point_to[0]
        and point[1] < point_to[1]
        and pipes[point].goes_east()
        and pipes[point_to].goes_west()
    ):
        return True
    elif (
        point[0] == point_to[0]
        and point[1] > point_to[1]
        and pipes[point].goes_west()
        and pipes[point_to].goes_east()
    ):
        return True
    else:
        return False


def blocks(
    pipes: dict[tuple[int, int], Tile],
    point: tuple[int, int],
    point_to: tuple[int, int],
):
    if (
        point[0] < point_to[0]
        and point[1] == point_to[1]
        and pipes[point_to].blocks_south_north()
    ):
        return True
    elif (
        point[0] > point_to[0]
        and point[1] == point_to[1]
        and pipes[point_to].blocks_south_north()
    ):
        return True
    elif (
        point[0] == point_to[0]
        and point[1] < point_to[1]
        and pipes[point_to].blocks_east_south()
    ):
        return True
    elif (
        point[0] == point_to[0]
        and point[1] > point_to[1]
        and pipes[point_to].blocks_east_south()
    ):
        return True
    else:
        return False


def parse_input() -> dict[tuple[int, int], Tile]:
    p = Path("inputs") / "day10"
    with p.open("r") as f:
        input = f.read()

    pipes = defaultdict(
        lambda: Tile.Ground,
        {
            (i, j): Tile.parse(v)
            for i, vs in enumerate(input.splitlines())
            for j, v in enumerate(vs)
        },
    )

    return pipes


def solve(pipes: dict[tuple[int, int], Tile]):
    s_i, s_j = next((i, j) for (i, j), v in pipes.items() if v is Tile.Start)

    shortest_paths = defaultdict(lambda: np.inf)

    shortest_paths[(s_i, s_j)] = 0

    next_steps = [
        ((i, j), 0)
        for i, j in [(s_i - 1, s_j), (s_i + 1, s_j), (s_i, s_j - 1), (s_i, s_j + 1)]
        if connects_to(pipes, (s_i, s_j), (i, j))
    ]

    while len(next_steps) > 0:
        pos, n_steps = next_steps.pop(0)
        p_i, p_j = pos

        if shortest_paths[pos] < n_steps:
            continue

        shortest_paths[pos] = n_steps + 1

        next_steps.extend(
            [
                ((i, j), n_steps + 1)
                for i, j in [
                    (p_i - 1, p_j),
                    (p_i + 1, p_j),
                    (p_i, p_j - 1),
                    (p_i, p_j + 1),
                ]
                if connects_to(pipes, pos, (i, j))
            ]
        )

    return shortest_paths


def scan_lines(
    pipes: dict[tuple[int, int], Tile], loop_tiles: set[tuple[int, int]]
) -> set[tuple[int, int]]:
    enclosed_pipes = []

    i_max = max(i for i, _ in pipes.keys())
    j_max = max(j for _, j in pipes.keys())

    for i in range(0, i_max):
        in_loop = False

        for j in range(0, j_max):
            pos = (i, j)

            if pos in loop_tiles and pipes[pos] in [
                Tile.NorthSouth,
                Tile.NorthEast,
                Tile.NorthWest,
            ]:
                in_loop = not in_loop

            if in_loop and pos not in loop_tiles:
                enclosed_pipes.append(pos)

    return enclosed_pipes


def solve_2(pipes: dict[tuple[int, int], Tile]):
    s_i, s_j = next((i, j) for (i, j), v in pipes.items() if v is Tile.Start)

    shortest_paths = solve(pipes)
    loop_tiles = set(shortest_paths.keys())

    enclosed_poses = scan_lines(pipes, loop_tiles)

    return len(enclosed_poses)


if __name__ == "__main__":
    pipes = parse_input()
    print(f"Part one: {max(solve(pipes).values())}")
    print(f"Part two: {solve_2(pipes)}")
