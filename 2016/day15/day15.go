package main

import (
  "fmt"
  "io/ioutil"
  "regexp"
  "strings"
  "strconv"
)

type Disc struct {
  numberOfPositions, currentPosition int
}

func main() {
  discs := parseInput("day15input")
  solve(discs)
}

func parseInput(filename string) []Disc {
  var discs []Disc
  input, err := ioutil.ReadFile(filename)
  r, _ := regexp.Compile("[0-9]+")
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine) - 1; i++ {
      read := r.FindAllString(inputLine[i], -1)
      numberOfPositions, _ := strconv.Atoi(read[1])
      startPosition, _ := strconv.Atoi(read[3])
      discs = append(discs, Disc{numberOfPositions, startPosition})
    }
  }
  return discs
}

func (d *Disc) tick() {
  d.currentPosition = (d.currentPosition + 1) % d.numberOfPositions
}

func solve(discs []Disc) {
  solved := false
  firstTime := 0
  for i := 0; !solved; i++ {
    allZero := true
    for j := 0; j < len(discs) && allZero; j++ {
      if (discs[j].currentPosition + (i+j+1)) % discs[j].numberOfPositions != 0 {
        allZero = false
      }
    }
    if allZero {
      solved = true
      firstTime = i
    }
  }
  fmt.Println(firstTime)
}
