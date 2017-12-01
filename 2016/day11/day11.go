package main

import (
  "fmt"
  "io/ioutil"
  //"strconv"
  "regexp"
  "strings"
)

type Generator struct {
  genType string
}

type MicroChip struct {
  chipType string
}

type Floor struct {
  chips, generators []string
}

func main() {
  floors := parseInput("day11input")
  move(floors)

}

func parseInput(filename string) []Floor {
  var floors []Floor
  input, err := ioutil.ReadFile(filename)
  r, _ := regexp.Compile("([a-z]+-?[a-z]+) (generator|microchip)")
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine); i++ {
      var generators []string
      var microchips []string
      items := r.FindAllStringSubmatch(inputLine[i], -1)
      for _, v := range items {
        if v[2] == "generator" {
          generators = append(generators, v[1])
        } else {
          microshipType := strings.Split(v[1], "-")
          microchips = append(microchips, microshipType[0])
        }
      }
      floors = append(floors, Floor{microchips, generators})
    }
  }
  return floors
}


func legalFloor(generators []Generator, microchips []MicroChip) bool {
  if len(microchips) == 0 {
    return true
  }
  var activeGenerators []Generator
  for i := 0; i < len(generators); i++ {
    active := true
    for j := 0; j < len(microchips); j++ {
      if microchips[j].chipType == generators[i].genType {
        active = false
      }
    }
    if active {
      activeGenerators = append(activeGenerators, generators[i])
    }
  }

  if len(activeGenerators) == 0 {
    return true
  } else {
    return false
  }
}

func move(floors []Floor) {
  count := 0
  for len(floors[3].generators) < 7 || len(floors[3].chips) < 7 {
    if (len(floors[0].generators) == 0 && len(floors[0].chips) == 0) {
      if (len(floors[1].generators) == 0 && len(floors[1].chips) == 0) {
        if (len(floors[2].generators) == 0 && len(floors[2].chips) == 0) {
          floors = moveDown(3, floors)
          count++
          if (len(floors[3].generators) != 0 || len(floors[3].chips) != 0) {
            floors = moveUp(3, floors)
            count++
          }
        } else {
          floors = moveDown(2, floors)
          count++
          if (len(floors[2].generators) != 0 || len(floors[2].chips) != 0) {
            floors = moveUp(2, floors)
            count++
          }
        }
      } else {
        floors = moveDown(1, floors)
        count++
        if (len(floors[1].generators) != 0 || len(floors[1].chips) != 0) {
          floors = moveUp(1, floors)
          count++
        }
      }
    } else {
      floors = moveDown(0, floors)
      count++
      if (len(floors[0].generators) != 0 || len(floors[0].chips) != 0) {
        floors = moveUp(0, floors)
        count++
      }
    }
  }
  fmt.Println(len(floors[3].generators) < 5 || len(floors[3].chips) < 5)
  fmt.Println(floors)
  fmt.Println(count)
}

func moveDown(i int, floors []Floor) []Floor {
  a, b := findPair(floors[i])
  if (a == -1 && len(floors[i].generators) == 0) {
    cut := 2
    if len(floors[i].chips) < 2 {
      cut = 1
    }
    // Move two chips down
    fmt.Println(floors[i].chips)
    downChips := floors[i].chips[:cut]
    floors[i].chips = floors[i].chips[cut:]
    floors[i+1].chips = append(floors[i+1].chips, downChips...)
  } else if a == -1 {
    cut := 2
    if len(floors[i].generators) < 2 {
      cut = 1
    }
    downGens := floors[i].generators[:cut]
    floors[i].generators = floors[i].generators[cut:]
    floors[i+1].generators = append(floors[i+1].generators, downGens...)
  } else {
    gen := floors[i].generators[a]
    chip := floors[i].chips[b]
    floors[i].generators = append(floors[i].generators[:a], floors[i].generators[a+1:]...)
    floors[i].chips = append(floors[i].chips[:b], floors[i].chips[b+1:]...)
    floors[i+1].chips = append(floors[i+1].chips, chip)
    floors[i+1].generators = append(floors[i+1].generators, gen)
  }
  return floors
}

func moveUp(i int, floors []Floor) []Floor {
  if len(floors[i+1].generators) == 0 {
    chip := floors[i+1].chips[0]
    floors[i+1].chips = floors[i+1].chips[1:]
    floors[i].chips = append(floors[i].chips, chip)
  } else {
    gen := floors[i+1].generators[0]
    floors[i+1].generators = floors[i+1].generators[1:]
    floors[i].generators = append(floors[i].generators, gen)
  }
  return floors
}

func findPair(floor Floor) (int, int) {
  for k, g := range floor.generators {
    for k1, c := range floor.chips {
      if g == c {
        return k, k1
      }
    }
  }
  return -1, -1
}
