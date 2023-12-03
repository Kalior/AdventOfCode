from dataclasses import dataclass
from pathlib import Path


@dataclass
class Revelation:
    n_blue: int
    n_red: int
    n_green: int

    def parse(group: str) -> "Revelation":
        revealed = group.split(", ")
        n_green = 0
        n_blue = 0
        n_red = 0
        for n_color in revealed:
            n, color = n_color.split(" ")
            if color == "green":
                n_green = int(n)
            elif color == "red":
                n_red = int(n)
            elif color == "blue":
                n_blue = int(n)

        return Revelation(n_blue, n_red, n_green)

    def power(self):
        return self.n_red * self.n_blue * self.n_green

@dataclass
class Game:
    game_id: int
    revelations: list[Revelation]

    def has_at_most(self, *, max_green: int, max_blue: int, max_red: int) -> bool:
        return all(
            reveal.n_green <= max_green
            and reveal.n_blue <= max_blue
            and reveal.n_red <= max_red
            for reveal in self.revelations
        )

    def smallest_possible_revelation(self) -> Revelation:
        max_red = max(r.n_red for r in self.revelatooions)
        max_blue = max(r.n_blue for r in self.revelations)
        max_green = max(r.n_green for r in self.revelations)
        return Revelation(max_blue, max_red, max_green)
    

    def parse(line: str) -> "Game":
        game, revelations = line.split(": ")
        game_id = int(game.replace("Game ", ""))

        revelations = [Revelation.parse(g) for g in revelations.split("; ")]

        return Game(game_id, revelations)


def parse_input() -> list[Game]:
    p = Path("inputs") / "day2"
    with p.open("r") as f:
        input = f.read()

    return [Game.parse(line) for line in input.splitlines()]


def solve(games: list[Game]):
    return sum([
        game.game_id
        for game in games
        if game.has_at_most(max_blue=14, max_green=13, max_red=12)
    ])

def solve_2(games: list[Game]):
    return sum([
        game.smallest_possible_revelation().power()
        for game in games
    ])


if __name__ == "__main__":
    games = parse_input()
    print(solve(games))
    print(solve_2(games))
