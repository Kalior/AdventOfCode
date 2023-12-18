from collections import defaultdict, namedtuple
from enum import Enum, auto
from pathlib import Path

from tqdm import tqdm

Pos = namedtuple("Pos", ["x", "y"])


class Dir(Enum):
    Up = auto()
    Down = auto()
    Left = auto()
    Right = auto()


def advance(pos: Pos, dir: Dir) -> tuple[Pos, Dir]:
    match dir:
        case Dir.Up:
            return (Pos(pos.x, pos.y - 1), dir)
        case Dir.Down:
            return (Pos(pos.x, pos.y + 1), dir)
        case Dir.Left:
            return (Pos(pos.x - 1, pos.y), dir)
        case Dir.Right:
            return (Pos(pos.x + 1, pos.y), dir)


def move(pos: Pos, dir: Dir, layout: dict[Pos, str]) -> list[tuple[Pos, Dir]]:
    match layout[pos], dir:
        case "/", Dir.Right:
            return [advance(pos, Dir.Up)]
        case "/", Dir.Left:
            return [advance(pos, Dir.Down)]
        case "/", Dir.Down:
            return [advance(pos, Dir.Left)]
        case "/", Dir.Up:
            return [advance(pos, Dir.Right)]
        case "\\", Dir.Right:
            return [advance(pos, Dir.Down)]
        case "\\", Dir.Left:
            return [advance(pos, Dir.Up)]
        case "\\", Dir.Up:
            return [advance(pos, Dir.Left)]
        case "\\", Dir.Down:
            return [advance(pos, Dir.Right)]
        case "-", (Dir.Up | Dir.Down):
            return [
                advance(pos, Dir.Left),
                advance(pos, Dir.Right),
            ]
        case "-", _:
            return [advance(pos, dir)]
        case "|", (Dir.Left | Dir.Right):
            return [
                advance(pos, Dir.Up),
                advance(pos, Dir.Down),
            ]
        case "|", _:
            return [advance(pos, dir)]
        case ".", _:
            return [advance(pos, dir)]
        case _:
            raise ValueError(f"{pos} ({layout[pos]}) and {dir} found no match")


def parse_input() -> dict[Pos, str]:
    p = Path("inputs") / "day16"
    with p.open("r") as f:
        input = f.read()

    layout = defaultdict(lambda: ".")

    for y, line in enumerate(input.splitlines()):
        for x, v in enumerate(line):
            layout[Pos(x, y)] = v

    return layout


def solve(
    layout: dict[Pos, str], start_pos: Pos = Pos(0, 0), start_dir: Dir = Dir.Right
):
    beams = [(start_pos, start_dir)]
    energized = set([(start_pos, start_dir)])

    while len(beams) > 0:
        beams = [
            (pos, dir)
            for beam in beams
            for (pos, dir) in move(*beam, layout=layout)
            if pos in layout.keys() and (pos, dir) not in energized
        ]
        energized = energized.union(set(p for p in beams))

    return len(set(p for p, _ in energized))


def solve_2(layout: dict[Pos, str]):
    max_x = max(p.x for p in layout.keys())
    max_y = max(p.y for p in layout.keys())

    corners = [
        (Pos(0, 0), Dir.Right),
        (Pos(0, 0), Dir.Down),
        (Pos(max_x, max_y), Dir.Up),
        (Pos(max_x, max_y), Dir.Left),
        (Pos(0, max_y), Dir.Up),
        (Pos(0, max_y), Dir.Right),
        (Pos(max_x, 0), Dir.Down),
        (Pos(max_x, 0), Dir.Left),
    ]
    start_positions = (
        [(Pos(x, 0), Dir.Down) for x in range(0, max_x)]
        + [(Pos(x, max_y), Dir.Up) for x in range(0, max_x)]
        + [(Pos(0, y), Dir.Right) for y in range(0, max_y)]
        + [(Pos(max_x, y), Dir.Left) for y in range(0, max_y)]
        + corners
    )

    return max([solve(layout, p, dir) for p, dir in tqdm(start_positions)])


if __name__ == "__main__":
    layout = parse_input()
    print(f"Part one: {solve(layout)}")
    print(f"Part two: {solve_2(layout)}")
