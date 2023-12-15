from pathlib import Path

import numpy as np


def _pattern_to_note(input: str) -> np.ndarray:
    return np.array(
        [np.array(list(v.strip()), dtype=object) for v in input.strip().splitlines()]
    )


def parse_input() -> list[np.ndarray]:
    p = Path("inputs") / "day13"
    with p.open("r") as f:
        input = f.read()

    return [_pattern_to_note(group) for group in input.split("\n\n")]


def reflection_at_column(
    note: np.ndarray, index: int, *, with_smudge: bool = False
) -> bool:
    dist_to_right_edge = note.shape[1] - index
    dist_to_left_edge = index

    n_columns_in_reflection = min(dist_to_left_edge, dist_to_right_edge)

    columns = note[:, index - n_columns_in_reflection : index + n_columns_in_reflection]

    left_reflection = columns[:, :n_columns_in_reflection]
    right_reflection = np.fliplr(columns[:, n_columns_in_reflection:])

    if with_smudge:
        return sum(~(left_reflection.flatten() == right_reflection.flatten())) == 1
    else:
        return np.array_equal(left_reflection, right_reflection)


def reflection_at_row(
    note: np.ndarray, index: int, *, with_smudge: bool = False
) -> bool:
    dist_to_right_edge = note.shape[0] - index
    dist_to_left_edge = index

    n_rows_in_reflection = min(dist_to_left_edge, dist_to_right_edge)

    rows = note[index - n_rows_in_reflection : index + n_rows_in_reflection, :]

    top_reflection = rows[:n_rows_in_reflection, :]
    bottom_reflection = np.flipud(rows[n_rows_in_reflection:, :])

    if with_smudge:
        return sum(~(top_reflection.flatten() == bottom_reflection.flatten())) == 1
    else:
        return np.array_equal(top_reflection, bottom_reflection)


def solve(notes: list[np.ndarray]):
    summary = 0
    for note in notes:
        for i in range(note.shape[0]):
            if reflection_at_row(note, i):
                summary += i * 100

        for i in range(note.shape[1]):
            if reflection_at_column(note, i):
                summary += i

    return summary


def solve_2(notes: list[np.ndarray]):
    summary = 0
    for note in notes:
        for i in range(note.shape[0]):
            if reflection_at_row(note, i, with_smudge=True):
                summary += i * 100

        for i in range(note.shape[1]):
            if reflection_at_column(note, i, with_smudge=True):
                summary += i

    return summary


if __name__ == "__main__":
    assert reflection_at_column(
        _pattern_to_note(
            """
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
            """
        ),
        5,
    )

    assert reflection_at_row(
        _pattern_to_note(
            """
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
            """
        ),
        4,
    )

    notes = parse_input()
    print(f"Part one: {solve(notes)}")
    print(f"Part two: {solve_2(notes)}")
