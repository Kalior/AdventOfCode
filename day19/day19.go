package main

import (
  "fmt"
  "math"
)

type Elf struct {
  number, presents int
}

func solve(elvesCount int) {
  var elves []Elf
  for i := 0; i < elvesCount; i++ {
    elves = append(elves, Elf{i+1, 1})
  }

  for i := 0; i < len(elves) && !onlyOne(elves); {
    if elves[i].presents != 0 {
      //fmt.Println(elves[i].number)
      elfIndex := nextElf(i, elves)
      elves[i].presents += elves[elfIndex].presents
      elves[elfIndex].presents = 0
      i = elfIndex
    } else {
      i = nextElf(i, elves)
    }
  }
  fmt.Println(onlyOne(elves))
  fmt.Println(lastElf(elves))
}

func main() {
  solve2(3005290)
}

func onlyOne(elves []Elf) bool {
  number := 0
  for i := 0; i < len(elves) && number <= 1; i++ {
    if elves[i].presents != 0 {
      number++;
    }
  }
  return number == 1
}

func nextElf(current int, elves []Elf) int {
  elfIndex := -1
  foundElf := false
  for j := current+1; j < len(elves) && !foundElf; j++ {
    if elves[j].presents != 0 {
      foundElf = true
      elfIndex = j
    }
  }
  for j := 0; j < current && !foundElf; j++ {
    if elves[j].presents != 0 {
      foundElf = true
      elfIndex = j
    }
  }
  return elfIndex
}

func lastElf(elves []Elf) Elf {
  elf := Elf{-1, -1}
  for i := 0; i < len(elves) && elf.number == -1; i++ {
    if elves[i].presents != 0 {
      elf = elves[i]
    }
  }
  return elf
}

func solve2(elvesCount int) {
  var elves []Elf
  for i := 0; i < elvesCount; i++ {
    elves = append(elves, Elf{i+1, 1})
  }
  remainingElves := elves
  for !onlyOne(elves) {
    removedBeforeCurrentIndex := 0
    removedElves := 0
    remainingElves = remainingElves[0:0]
    for i := 0; i < len(elves); i++ {
      if elves[i].presents != 0 {
        remainingElves = append(remainingElves, elves[i])
      }
    }
    elves = remainingElves
    for i := 0; i < len(elves); i++ {
      if elves[i].presents != 0 {
        half := int(math.Floor(float64(len(elves)-removedElves)/2))
        elfIndex := (i + half + removedElves - removedBeforeCurrentIndex) % len(elves)
        removedElves++
        elves[i].presents += elves[elfIndex].presents
        elves[elfIndex].presents = 0
      } else {
        removedBeforeCurrentIndex++
      }
    }
    fmt.Println(len(elves))
  }
  fmt.Println(lastElf(elves))
}
