package main

import (
  "fmt"
  "strconv"
)

type Position struct {
  x, y int
}

func main() {
  walls := calcWalls()
  walk(walls)
}

func calcWalls() map[Position]bool {
  walls := make(map[Position]bool)
  for i := 0; i < 50; i++ {
    for j := 0; j < 50; j++ {
      p := Position{i, j}
      n := strconv.FormatInt(int64(p.x*p.x + 3*p.x + 2*p.x*p.y + p.y + p.y*p.y + 1362), 2)
      oneCount := 0
      fmt.Println(n)
      for i := 0; i < len(n); i++ {
        if n[i] == '1' {
          oneCount++
        }
      }
      if oneCount % 2 == 1 {
        walls[p] = true
      } else {
        walls[p] = false
      }
    }
  }
  return walls
}

func walk(walls map[Position]bool) {
  dest := Position{31,39}
  shortest := make(map[Position]int)
  for i := 0; i < 50; i++ {
    for j := 0; j < 50; j++ {
      if i == 1 && j == 1 {
        shortest[Position{i,j}] = 0
      } else {
        shortest[Position{i,j}] = 9999
      }
    }
  }
  recWalk(Position{1,1}, 0, walls, &shortest)
  fmt.Println(shortest[dest])

  reached := 0
  for i := 0; i < 50; i++ {
    for j := 0; j < 50; j++ {
      if shortest[Position{i,j}] < 9999 {
        reached++
      }
    }
  }
  fmt.Println(reached)
}

func recWalk(p Position, length int, walls map[Position]bool, shortest *map[Position]int) {
  if (*shortest)[p] > length && length <= 50 {
    (*shortest)[p] = length
    length += 1
    if !walls[Position{p.x, p.y+1}] {
      recWalk(Position{p.x, p.y+1}, length, walls, shortest)
    }
    if !walls[Position{p.x, p.y-1}] {
      recWalk(Position{p.x, p.y-1}, length, walls, shortest)
    }
    if !walls[Position{p.x+1, p.y}] {
      recWalk(Position{p.x+1, p.y}, length, walls, shortest)
    }
    if !walls[Position{p.x-1, p.y}] {
      recWalk(Position{p.x-1, p.y}, length, walls, shortest)
    }
  }
}
