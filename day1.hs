directions = [L 3, R 1, L 4, L 1, L 2, R 4, L 3, L 3, R 2, R 3, L 5, R 1, R 3, L 4, L 1, L 2, R 2, R 1, L 4, L 4, R 2, L 5, R 3, R 2, R 1, L 1, L 2, R 2, R 2, L 1, L 1, R 2, R 1, L 3, L 5, R 4, L 3, R 3, R 3, L 5, L 190, L 4, R 4, R 51, L 4, R 5, R 5, R 2, L 1, L 3, R 1, R 4, L 3, R 1, R 3, L 5, L 4, R 2, R 5, R 2, L 1, L 5, L 1, L 1, R 78, L 3, R 2, L 3, R 5, L 2, R 2, R 4, L 1, L 4, R 1, R 185, R 3, L 4, L 1, L 1, L 3, R 4, L 4, L 1, R 5, L 5, L 1, R 5, L 1, R 2, L 5, L 2, R 4, R 3, L 2, R 3, R 1, L 3, L 5, L 4, R 3, L 2, L 4, L 5, L 4, R 1, L 1, R 5, L 2, R 4, R 2, R 3, L 1, L 1, L 4, L 3, R 4, L 3, L 5, R 2, L 5, L 1, L 1, R 2, R 3, L 5, L 3, L 2, L 1, L 4, R 4, R 4, L 2, R 3, R 1, L 2, R 1, L 2, L 2, R 3, R 3, L 1, R 4, L 5, L 3, R 4, R 4, R 1, L 2, L 5, L 3, R 1, R 4, L 2, R 5, R 4, R 2, L 5, L 3, R 4, R 1, L 1, R 5, L 3, R 1, R 5, L 2, R 1, L 5, L 2, R 2, L 2, L 3, R 3, R 3, R 1]

data ElfDirection = R Integer | L Integer

type Dir = (CurrentDirection, Integer, Integer)

data CurrentDirection = East | West | South | North

count = show $ abs upDown + abs leftRight
  where
  (_, upDown, leftRight) = foldl move (North, 0, 0) directions

move :: Dir -> ElfDirection -> Dir
move (currentDirection, upDown, leftRight) (R i) = case currentDirection of
  South -> (West, upDown, leftRight - i)
  West -> (North, upDown + i, leftRight)
  East -> (South, upDown - i, leftRight)
  North -> (East, upDown, leftRight + i)
move (currentDirection, upDown, leftRight) (L i) = case currentDirection of
  South -> (East, upDown, leftRight + i)
  West -> (South, upDown - i, leftRight)
  East -> (North, upDown + i, leftRight)
  North -> (West, upDown, leftRight - i)
