from pathlib import Path
import numpy as np
from dataclasses import dataclass
import functools
from collections import Counter

use_jokers = False


@dataclass(unsafe_hash=True)
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
            # "J": 12,
            "T": 11,
            "9": 9,
            "8": 8,
            "7": 7,
            "6": 6,
            "5": 5,
            "4": 4,
            "3": 3,
            "2": 2,
            "J": 1,
        }
        self_v = card_to_v[self.val]
        other_v = card_to_v[other.val]
        return self_v < other_v


@dataclass
@functools.total_ordering
class Hand:
    cards: list[Card]

    @staticmethod
    def from_str(cards: str) -> "Hand":
        return Hand([Card(c) for c in cards])

    @functools.cached_property
    def counts(self) -> list[tuple[Card, int]]:
        counts = Counter(self.cards).most_common()
        return counts

    @property
    def n_jokers(self):
        if use_jokers:
            return sum(c.val == "J" for c in self.cards)
        else:
            return 0

    def __str__(self) -> str:
        return "".join([c.val for c in self.cards])

    def __eq__(self, other):
        return all(c == o for c, o in zip(self.cards, other.cards))

    def is_five_of_a_kind(self):
        return any(
            (v + j) == 5
            for k, v in self.counts
            for j in range(self.n_jokers + 1)
            if k.val != "J"
        ) or any(v == 5 for k, v in self.counts if k.val == "J")

    def is_four_of_a_kind(self):
        return any(
            (v + j) == 4
            for k, v in self.counts
            for j in range(self.n_jokers + 1)
            if k.val != "J"
        ) or any(v == 4 for k, v in self.counts if k.val == "J")

    def is_full_house(self):
        no_jokers = any(v == 3 for _, v in self.counts) and any(
            v == 2 for _, v in self.counts
        )
        if no_jokers:
            return True

        for j in range(self.n_jokers + 1):
            other_j = self.n_jokers - j
            if any(
                (v + j) == 3 and (v2 + other_j) == 2
                for k, v in self.counts
                for k2, v2 in self.counts
                if k.val != "J" and k2.val != "J" and k != k2
            ):
                return True

        return False

    def is_three_of_a_kind(self):
        return any(
            (v + j) == 3
            for k, v in self.counts
            for j in range(self.n_jokers + 1)
            if k.val != "J"
        ) or any(v == 3 for k, v in self.counts if k.val == "J")

    def is_two_pair(self):
        no_jokers = sum(v == 2 for _, v in self.counts) == 2
        if no_jokers:
            return True

        for j in range(self.n_jokers + 1):
            other_j = self.n_jokers - j
            if any(
                v + j >= 2 and v_2 + other_j >= 2
                for k, v in self.counts
                for k_2, v_2 in self.counts
                if k_2 != k and k.val != "J" and k_2.val != "J"
            ):
                return True
        return False

    def is_one_pair(self):
        return any(
            (v + j) == 2
            for k, v in self.counts
            for j in range(self.n_jokers + 1)
            if k.val != "J"
        ) or any(v == 2 for k, v in self.counts if k.val == "J")

    def is_high_card(self):
        return all(v == 1 for _, v in self.counts)

    def has_first_higher_card(self, other):
        for c, o in zip(self.cards, other.cards):
            if c == o:
                continue
            return c > o

    def __lt__(self, other):
        # All of the checks to test
        ordered_checks = [
            Hand.is_five_of_a_kind,
            Hand.is_four_of_a_kind,
            Hand.is_full_house,
            Hand.is_three_of_a_kind,
            Hand.is_two_pair,
            Hand.is_one_pair,
            Hand.is_high_card,
        ]
        for check in ordered_checks:
            if check(self) and check(other):
                return not self.has_first_higher_card(other)
            elif check(self):
                return False
            elif check(other):
                return True

        raise ValueError


def parse_input() -> list[tuple[Hand, int]]:
    p = Path("inputs") / "day7"
    with p.open("r") as f:
        input = f.read()

    lines = input.split("\n")

    def _parse_line(line) -> tuple[Hand, int]:
        cards, bid = line.split()

        hand = Hand([Card(c) for c in cards])
        return hand, int(bid)

    return [_parse_line(line) for line in lines]


def solve(hands_with_bids: list[tuple[Hand, int]]):
    sorted_hands = list((sorted(hands_with_bids, key=lambda hand_bids: hand_bids[0])))

    return sum((1 + i) * bid for i, (_, bid) in enumerate(sorted_hands))


if __name__ == "__main__":
    hands = parse_input()
    print(f"Part one: {solve(hands)}")
    use_jokers = True

    smallest = Hand.from_str("32T3K")
    right_two_pair = Hand.from_str("KTJJT")

    assert smallest < right_two_pair

    assert smallest.is_one_pair()
    assert right_two_pair.is_two_pair()

    two_pair = Hand.from_str("KK677")

    assert two_pair.is_two_pair()
    # assert right_two_pair < two_pair

    assert Hand.from_str("T55J5").is_three_of_a_kind()
    assert Hand.from_str("T55J5").is_full_house()

    assert Hand.from_str("JJJJJ").is_five_of_a_kind()

    assert Hand.from_str("TJ3JJ").n_jokers == 3
    assert not Hand.from_str("TJ3JJ").is_five_of_a_kind()
    assert Hand.from_str("TJ3JJ") < Hand.from_str("JJJJJ")

    assert Hand.from_str("J2K77").is_three_of_a_kind()
    assert not Hand.from_str("J2K77").is_four_of_a_kind()
    assert not Hand.from_str("J2K77").is_five_of_a_kind()
    assert not Hand.from_str("J2K77").is_full_house()

    assert Hand.from_str("J47JT").is_three_of_a_kind()
    assert not Hand.from_str("J47JT").is_four_of_a_kind()
    assert not Hand.from_str("J47JT").is_five_of_a_kind()
    assert not Hand.from_str("J47JT").is_full_house()

    assert not Hand.from_str("J2K77").has_first_higher_card(Hand.from_str("J47JT"))
    assert Hand.from_str("J2K77") < Hand.from_str("J47JT")

    assert Hand.from_str("J47JT") < Hand.from_str("J5859") < Hand.from_str("J5KJ8")

    print(f"Part two: {solve(hands)}")
