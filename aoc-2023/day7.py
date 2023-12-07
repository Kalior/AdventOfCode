import functools
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

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
        if not use_jokers:
            card_to_v["J"] = 12

        self_v = card_to_v[self.val]
        other_v = card_to_v[other.val]
        return self_v < other_v


joker_card = Card("J")


@dataclass
@functools.total_ordering
class Hand:
    cards: list[Card]

    @staticmethod
    def from_str(cards: str) -> "Hand":
        return Hand([Card(c) for c in cards])

    @functools.cached_property
    def counts(self) -> list[tuple[Card, int]]:
        counts = Counter(self.cards)
        if joker_card in counts.keys() and use_jokers:
            n_jokers = counts[joker_card]
            counts.pop(joker_card)
            k, v = counts.most_common(1)[0]
            counts[k] = v + n_jokers

        return {v: k for v, k in counts.most_common()}

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
        return self.n_jokers == 5 or 5 in self.counts.values()

    def is_four_of_a_kind(self):
        return 4 in self.counts.values()

    def is_full_house(self):
        return 3 in self.counts.values() and 2 in self.counts.values()

    def is_three_of_a_kind(self):
        return 3 in self.counts.values()

    def is_two_pair(self):
        return sum(v == 2 for _, v in self.counts.items()) == 2

    def is_one_pair(self):
        return 2 in self.counts.values()

    def is_high_card(self):
        return all(v == 1 for _, v in self.counts.items())

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
                return other.has_first_higher_card(self)
            elif check(self):
                return False
            elif check(other):
                return True

        raise ValueError("No valid sorting found")


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
    sorted_hands = sorted(hands_with_bids, key=lambda hand_bids: hand_bids[0])

    return sum((1 + i) * bid for i, (_, bid) in enumerate(sorted_hands))


if __name__ == "__main__":
    print(f"Part one: {solve(parse_input())}")
    use_jokers = True
    print(f"Part two: {solve(parse_input())}")
