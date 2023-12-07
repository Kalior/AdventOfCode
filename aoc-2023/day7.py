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

    @property
    def counts(self) -> list[tuple[Card, int]]:
        counts = Counter(self.cards).most_common()
        return counts
        # if not use_jokers:
        #     return counts
        # else:
        #     return [(k, v) for k, v in counts if k.val != "J"]

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
            if any((v + j) == 3 for k, v in self.counts if k.val != "J") and any(
                (v + other_j) == 2 for k, v in self.counts if k.val != "J"
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
            for k, v in self.counts:
                for k_2, v_2 in self.counts:
                    if k_2 != k and k.val != "J":
                        if v + j >= 2 and v_2 + other_j >= 2:
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

    def __le__(self, other):
        return self < other and other == self

    def __ge__(self, other):
        return other < self and other == self

    def __gt__(self, other):
        return other < self

    def __lt__(self, other):
        # Five of a kind checks
        ordered_checks = [
            (self.is_five_of_a_kind(), other.is_five_of_a_kind(), "is_five_of_a_kind"),
            (self.is_four_of_a_kind(), other.is_four_of_a_kind(), "is_four_of_a_kind"),
            (self.is_full_house(), other.is_full_house(), "is_full_house"),
            (
                self.is_three_of_a_kind(),
                other.is_three_of_a_kind(),
                "is_three_of_a_kind",
            ),
            (self.is_two_pair(), other.is_two_pair(), "is_two_pair"),
            (self.is_one_pair(), other.is_one_pair(), "is_one_pair"),
            (self.is_high_card(), other.is_high_card(), "is_high_card"),
        ]
        for check, other_check, name in ordered_checks:
            # if name == "is_three_of_a_kind" and other_check and other.n_jokers > 0:
            #     print(other)
            # if name == "is_three_of_a_kind" and check and self.n_jokers > 0:
            #     print(self)
            # print(f"{check}, {other_check}, {name}")
            if check and other_check:
                return not self.has_first_higher_card(other)
            elif check:
                return False
            elif other_check:
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
    print("\n".join([f"{h}, {b}" for h, b in sorted_hands if h.n_jokers > 1]))

    return sum((1 + i) * bid for i, (_, bid) in enumerate(sorted_hands))


if __name__ == "__main__":
    hands = parse_input()
    #    print(f"Part one: {solve(hands)}")
    use_jokers = True

    smallest = Hand.from_str("32T3K")
    right_two_pair = Hand.from_str("KTJJT")

    assert smallest < right_two_pair

    assert smallest.is_one_pair()
    assert right_two_pair.is_two_pair()

    two_pair = Hand.from_str("KK677")

    assert two_pair.is_two_pair()
    # assert right_two_pair < two_pair

    full_house = Hand.from_str("T55J5")

    assert full_house.is_three_of_a_kind()
    assert full_house.is_full_house()

    all_jokers = Hand.from_str("JJJJJ")

    assert all_jokers.is_five_of_a_kind()

    not_all_jokers = Hand.from_str("TJ3JJ")

    assert not_all_jokers.n_jokers == 3
    assert not not_all_jokers.is_five_of_a_kind()
    assert not_all_jokers < all_jokers

    j_k = Hand.from_str("JKKK2")

    print(f"Part two: {solve(hands)}")
