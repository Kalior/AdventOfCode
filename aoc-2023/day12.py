from pathlib import Path
import functools

import numpy as np

Record = tuple[str, tuple[int]]


def parse_input() -> list[Record]:
    p = Path("inputs") / "day12"
    with p.open("r") as f:
        input = f.read()

    def _parse_line(line) -> Record:
        damaged_record, criteria = line.split(" ")
        criteria = tuple([int(v) for v in criteria.split(",")])

        return damaged_record, criteria

    return [_parse_line(line) for line in input.splitlines()]


@functools.lru_cache()
def record_permutations(rec: Record) -> int:
    damaged_record, criteria = rec

    if len(criteria) == 0 and any(v in ["#"] for v in damaged_record):
        # We didn't cover all of the areas with damages
        return 0
    elif len(damaged_record) == 0 and len(criteria) > 0:
        # We didn't place all of the damaged areas
        return 0
    elif len(criteria) == 0 and "#" not in damaged_record:
        return 1

    next_group = criteria[0]

    def _group_ends(i: int) -> bool:
        return (
            i + next_group < len(damaged_record)
            and damaged_record[i + next_group] != "#"
        )

    def _is_last_pos(i: int) -> bool:
        return i + next_group == len(damaged_record)

    next_possible_pos = next(
        (
            i
            for i in range(len(damaged_record))
            if i + next_group <= len(damaged_record)
            and "." not in damaged_record[i : i + next_group]
            and (_group_ends(i) or _is_last_pos(i))
        ),
        None,
    )
    if next_possible_pos is None or "#" in damaged_record[:next_possible_pos]:
        return 0
    else:
        n_permuts = record_permutations(
            (
                damaged_record[next_possible_pos + next_group + 1 :],
                criteria[1:],
            )
        )

        no_damaged_in_skipped = "#" not in damaged_record[: next_possible_pos + 1]
        if no_damaged_in_skipped:
            n_permuts += record_permutations(
                (
                    damaged_record[next_possible_pos + 1 :],
                    criteria,
                )
            )
        return n_permuts


def solve(records: list[Record]):
    return sum(record_permutations(record) for record in records)


def solve_2(records: list[Record]):
    return sum(
        record_permutations(("?".join([record] * 5), criteria * 5))
        for record, criteria in records
    )


if __name__ == "__main__":
    records = parse_input()
    assert record_permutations((".", ())) == 1
    assert record_permutations((".", (2,))) == 0
    assert record_permutations(("#", (1,))) == 1
    assert record_permutations(("???.###", (1, 1, 3))) == 1
    assert record_permutations(("?#?#?#?#?#?#?#?", (1, 3, 1, 6))) == 1
    assert record_permutations((".??..??...?##.", (1, 1, 3))) == 4
    assert record_permutations(("????.######..#####.", (1, 6, 5))) == 4
    assert record_permutations(("?###????????", (3, 2, 1))) == 10, record_permutations(
        ("?###????????", [3, 2, 1])
    )
    assert record_permutations(("#?#????????.?#.", (4, 1, 2, 1))) == 6
    assert record_permutations(("??????.?????", (1, 1, 1, 1))) == 86

    assert record_permutations(("??#?.#?.#?#????.", (1, 1, 7))) == 1

    print(f"Part one: {solve(records)}")
    print(f"Part two: {solve_2(records)}")
