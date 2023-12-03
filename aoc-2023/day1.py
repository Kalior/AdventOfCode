from pathlib import Path
import re


def parse_input() -> list[str]:
    p = Path("inputs") / "day1"
    with p.open("r") as f:
        input = f.read()

    return input.splitlines()


def _parse_int(integer: str) -> int:
    try:
        return int(integer)
    except ValueError as _:
        convert = dict(
            zip(
                [
                    "one",
                    "two",
                    "three",
                    "four",
                    "five",
                    "six",
                    "seven",
                    "eight",
                    "nine",
                ],
                range(1, 10),
            )
        )
        return convert[integer]


def _parse_calibration(line: str, pattern: re.Pattern) -> int:
    # Find a first match
    first_match = pattern.search(line)
    if first_match is None:
        return 0
    else:
        matches = list(pattern.finditer(line))

        first_match = matches[0][0]
        second_match = matches[-1]

        # There could be a rouge oneight or twone at the end. Try to find it by searching from where the last match is.
        second_match_pos = second_match.span()
        rouge_match = pattern.search(line, pos=second_match_pos[0] + 1)

        if rouge_match is not None:
            second = rouge_match[0]
        else:
            second = second_match[0]

        return _parse_int(first_match) * 10 + _parse_int(second)


def solve():
    calibration_lines = parse_input()

    pattern_one = re.compile(r"[1-9]")
    pattern_two = re.compile(r"[1-9]|one|two|three|four|five|six|seven|eight|nine")

    print(sum([_parse_calibration(line, pattern_one) for line in calibration_lines]))
    print(sum([_parse_calibration(line, pattern_two) for line in calibration_lines]))


if __name__ == "__main__":
    solve()
