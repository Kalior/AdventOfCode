from pathlib import Path
import numpy as np
from dataclasses import dataclass
import functools

import seaborn as sns
import matplotlib.pyplot as plt

@dataclass
@functools.total_ordering
class Card:
    val: str

    def __eq__(self, other):
        return self.val == other.val

    def __lt__(self, other):
        card_to_v = {
            "A": 15,
            "K": 14,
            "Q": 13,
            "J": 12,
            "T": 11,
        }
        if self.val.isnumeric() and other.val.isnumeric():
            return int(self.val) < int(other.val)
        elif self.val.isnumeric():
            return True
        elif other.val.isnumeric():
            return False
        else:
            self_v = card_to_v[self.val]
            other_v = card_to_v[self.val]
            return self_v < other_v

@dataclass
@functools.total_ordering
class Hand:
    cards: list[Card]

    def __eq__(self, other):
        return all(c == o for c, o in zip(self.cards, other.cards))

    def is_five_of_a_kind(self):
        pass

    def is_four_of_a_kind(self):
        pass

    def is_full_house(self):
        pass

    def is_three_of_a_kind(self):
        pass

    def is_two_pair(self):
        pass

    def is_one_pair(self):
        pass

    def is_high_card(self):
        pass



def parse_input() -> list[tuple[int, int]]:
    p = Path("inputs") / "day6"
    with p.open("r") as f:
        input = f.read()

    lines = input.split("\n")
    times = [int(time) for time in lines[0].replace("Time:", "").strip().split()]
    distances = [
        int(time) for time in lines[1].replace("Distance:", "").strip().split()
    ]

    return list(zip(times, distances))


def solve(record_times: list[tuple[int, int]]):
    pass


def solve_2(record_times: list[tuple[int, int]]):
    pass


if __name__ == "__main__":
    sns.set(style="whitegrid")
    games = parse_input()
    print(f"Part one: {solve(games)}")
    print(f"Part two: {solve_2(games)}")
