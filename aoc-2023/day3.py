from pathlib import Path
from itertools import count
from dataclasses import dataclass, field
from typing import Iterable
from math import prod

from more_itertools import split_at


@dataclass
class Entry:
    val: str
    identifier: int = field(default_factory=count().__next__)

    def __hash__(self):
        return hash(self.identifier)


@dataclass
class EngineSchematic:
    map: dict[tuple[int, int], Entry]

    def at(self, x: int, y: int) -> str:
        return self.map.get((x, y), ".")

    def max_x(self):
        return max([x for (x, _) in self.map.keys()])

    def max_y(self):
        return max([y for (_, y) in self.map.keys()])

    def adjacent_positions(self, x: int, y: int) -> Iterable[str]:
        for x_ in [x - 1, x, x + 1]:
            for y_ in [y - 1, y, y + 1]:
                yield self.at(x_, y_)

    def adjacent_numbers(self, x: int, y: int) -> list[int]:
        all_entries = [c for c in self.adjacent_positions(x, y) if c.val.isnumeric()]
        entries = list(set(all_entries))

        return [int(e.val) for e in entries]

    def parse(lines: str) -> "EngineSchematic":
        def _parse_line(line: str) -> Iterable[tuple[int, Entry]]:
            splits = split_at(
                enumerate(line),
                pred=lambda v: not v[1].isnumeric(),
                keep_separator=True,
            )
            for split in splits:
                value = Entry("".join(v for _, v in split))
                for x_, _ in split:
                    yield x_, value

        map = {
            (x, y): c
            for y, line in enumerate(lines.splitlines())
            for x, c in _parse_line(line)
        }

        return EngineSchematic(map)


def parse_input() -> EngineSchematic:
    p = Path("inputs") / "day3"
    with p.open("r") as f:
        input = f.read()

    return EngineSchematic.parse(input)


def solve(schema: EngineSchematic):
    part_number_sum = 0
    for x in range(0, schema.max_x()):
        for y in range(0, schema.max_y()):
            if schema.at(x, y).val.isnumeric() or schema.at(x, y).val == ".":
                continue
            else:
                part_number_sum += sum(schema.adjacent_numbers(x, y))

    return part_number_sum


def solve_2(schema: EngineSchematic):
    part_number_sum = 0
    for x in range(0, schema.max_x()):
        for y in range(0, schema.max_y()):
            if schema.at(x, y).val.isnumeric() or schema.at(x, y).val == ".":
                continue
            elif schema.at(x, y).val == "*":
                adjacent = schema.adjacent_numbers(x, y)
                if len(adjacent) == 2:
                    part_number_sum += prod(adjacent)

    return part_number_sum


if __name__ == "__main__":
    games = parse_input()
    print(solve(games))
    print(solve_2(games))
