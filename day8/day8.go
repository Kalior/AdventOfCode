package main

import (
  "fmt"
  "strings"
  "io/ioutil"
  "strconv"
  "regexp"
)

type Instruction interface {
  performInstruction(matrix [][]bool)
}

type RectInstruction struct {
  a, b int
}

type RowInstruction struct {
  a, b int
}

type ColumnInstruction struct {
  a, b int
}

func main() {
  matrix := make([][]bool, 6)
  for i := 0; i < 6; i++ {
    matrix[i] = make([]bool, 50)
  }
  instructions := parseInput("day8input")
  performInstructions(matrix, instructions)
  fmt.Println(count(matrix))
  print_matrix(matrix)
}

func parseInput(filename string) []Instruction {
  day8input, err := ioutil.ReadFile(filename)
  var instructions []Instruction
  r, _ := regexp.Compile("[0-9]+")
  if err != nil {
    fmt.Println(err)
  } else {
    day8lines := strings.Split(string(day8input), "\n")
    for i := 0; i < len(day8lines) - 1; i++ {
      line := strings.Split(day8lines[i], " ")
      if line[0] == "rect" {
        ab := r.FindAllString(line[1], -1)
        a, _ := strconv.Atoi(ab[0])
        b, _ := strconv.Atoi(ab[1])
        instructions = append(instructions, RectInstruction{a, b})
      } else if line[1] == "row" {
        astr := r.FindString(line[2])
        a, _ := strconv.Atoi(astr)
        b, _ := strconv.Atoi(line[4])
        instructions = append(instructions, RowInstruction{a, b})
      } else {
        astr := r.FindString(line[2])
        a, _ := strconv.Atoi(astr)
        b, _ := strconv.Atoi(line[4])
        instructions = append(instructions, ColumnInstruction{a, b})
      }
    }
  }
  return instructions
}

func performInstructions(matrix [][]bool, instructions []Instruction) {
  for i := 0; i < len(instructions); i++ {
    instructions[i].performInstruction(matrix)
  }
}

func (r RectInstruction) performInstruction(matrix [][]bool) {
  for i := 0; i < r.a; i++ {
    for j := 0; j < r.b; j++ {
      matrix[j][i] = true
    }
  }
}

func (c ColumnInstruction) performInstruction(matrix [][]bool) {
  prevValue := false
  for j := 0; j < c.b; j++ {
    for i := 0; i < 6; i++ {
      tmp := matrix[i][c.a]
      matrix[i][c.a] = prevValue
      prevValue = tmp
    }
    matrix[0][c.a] = prevValue
  }
}

func (r RowInstruction) performInstruction(matrix [][]bool) {
  prevValue := false
  for j := 0; j < r.b; j++ {
    for i := 0; i < 50; i++ {
      tmp := matrix[r.a][i]
      matrix[r.a][i] = prevValue
      prevValue = tmp
    }
    matrix[r.a][0] = prevValue
  }
}

func count(matrix [][]bool) int {
  count := 0
  for i := 0; i < len(matrix); i++ {
    for j := 0; j < len(matrix[i]); j++ {
      if matrix[i][j] {
        count++
      }
    }
  }
  return count
}

func print_matrix(matrix [][]bool) {
  for i := 0; i < len(matrix); i++ {
    for j := 0; j < len(matrix[i]); j++ {
      if matrix[i][j] {
        fmt.Print("#")
      } else {
        fmt.Print(" ")
      }
    }
    fmt.Print("\n")
  }
}
