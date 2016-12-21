package main

import (
  "fmt"
  "io/ioutil"
  "strconv"
  "strings"
)

type Instruction interface {
  perform([]byte) []byte
}

type SwapPosition struct {
  x, y int
}
func (i SwapPosition) perform(input []byte) []byte {
  input[i.x], input[i.y] = input[i.y], input[i.x]
  return input
}

type SwapLetter struct {
  x, y byte
}
func (i SwapLetter) perform(input []byte) []byte {
  index1, index2 := -1, -1
  for j := 0; j < len(input) && (index1 == -1 || index2 == -1); j++ {
    if input[j] == i.x {
      index1 = j
    }
    if input[j] == i.y {
      index2 = j
    }
  }
  input[index1], input[index2] = input[index2], input[index1]
  return input
}

type RotateSteps struct {
  steps int
  left bool
}
func (i RotateSteps) perform(input []byte) []byte {
  if i.left {
    for j := 0; j < i.steps; j++ {
      val := input[0]
      input = input[1:]
      input = append(input, val)
    }
  } else {
    for j := 0; j < i.steps; j++ {
      input = append([]byte{input[len(input)-1]}, input[:len(input)-1]...)
    }
  }
  return input
}

type RotatePosition struct {
  letter byte
}
func (i RotatePosition) perform(input []byte) []byte {
  index := -1
  for j := 0; j < len(input) && index == -1; j++ {
    if input[j] == i.letter {
      index = j
    }
  }
  if index >= 4 {
    index++
  }
  r := RotateSteps{index+1, false}
  return r.perform(input)
}

type ReversePositions struct {
  x,y int
}
func (i ReversePositions) perform(input []byte) []byte {
  for j := 0; j <= (i.y - i.x)/2; j++ {
    input[j+i.x], input[i.y-j] = input[i.y-j], input[j+i.x]
  }
  return input
}

type Move struct {
  from, to int
}
func (i Move) perform(input []byte) []byte {
  val := input[i.from]
  input = append(input[:i.from], input[i.from+1:]...)

  input = append(input, 0)
  copy(input[i.to+1:], input[i.to:])
  input[i.to] = val
  return input
}

func parseInput(filename string) []Instruction {
  var instructions []Instruction
  input, err := ioutil.ReadFile(filename)
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine); i++ {
      items := strings.Split(string(inputLine[i]), " ")
      if items[0] == "swap" && items[1] == "position" {
        x, _ := strconv.Atoi(items[2])
        y, _ := strconv.Atoi(items[5])
        instructions = append(instructions, SwapPosition{x, y})
      } else if items[0] == "swap" && items[1] == "letter" {
        instructions = append(instructions, SwapLetter{items[2][0], items[5][0]})
      } else if items[0] == "rotate" {
        if items[1] == "right" {
          steps, _ := strconv.Atoi(items[2])
          instructions = append(instructions, RotateSteps{steps, false})
        } else if items[1] == "left" {
          steps, _ := strconv.Atoi(items[2])
          instructions = append(instructions, RotateSteps{steps, true})
        } else {
          instructions = append(instructions, RotatePosition{items[6][0]})
        }
      } else if items[0] == "reverse" {
        x, _ := strconv.Atoi(items[2])
        y, _ := strconv.Atoi(items[4])
        instructions = append(instructions, ReversePositions{x, y})
      } else if items[0] == "move" {
        x, _ := strconv.Atoi(items[2])
        y, _ := strconv.Atoi(items[5])
        instructions = append(instructions, Move{x, y})
      }
    }
  }
  return instructions
}

func main() {
  instructions := parseInput("day21input")
  input := []byte("abcdefgh")

  for i := 0; i < len(instructions); i++ {
    input = instructions[i].perform(input)
  }
  fmt.Println("Task 1: ", string(input))
  permut := permutations(input)
  found := false
  for i := 0; i < len(permut) && !found; i++ {
    try := permut[i]
    for i := 0; i < len(instructions); i++ {
      try = instructions[i].perform(try)
    }
    if "fbgdceah" == string(try) {
      found = true
      fmt.Println("Task 2: ", string(permut[i]))
    }
  }
}

func permutations(arr []byte)[][]byte{
  var helper func([]byte, int)
  res := [][]byte{}

  helper = func(arr []byte, n int){
    if n == 1{
      tmp := make([]byte, len(arr))
      copy(tmp, arr)
      res = append(res, tmp)
    } else {
      for i := 0; i < n; i++{
        helper(arr, n - 1)
        if n % 2 == 1{
          tmp := arr[i]
          arr[i] = arr[n - 1]
          arr[n - 1] = tmp
        } else {
          tmp := arr[0]
          arr[0] = arr[n - 1]
          arr[n - 1] = tmp
        }
      }
    }
  }
  helper(arr, len(arr))
  return res
}
