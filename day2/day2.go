package main

import (
  "bytes"
  "fmt"
  "strconv"
  "io/ioutil"
  "strings"
)

func main() {
  stringCode := code()
  fmt.Println(stringCode)
}

func code() string{
  day2input, err := ioutil.ReadFile("day2input")
  if err != nil {
    return "error"
  }
  instructions := strings.Split(string(day2input), "\n")
  fmt.Println(instructions)
  var code [5]int
  currentPosition := 5
  for i := 0; i < len(instructions) - 1; i++ {
    for j := 0; j < len(instructions[i]); j++ {
      currentPosition = move(string(instructions[i][j]), currentPosition)
    }
    code[i] = currentPosition
  }
  var codeStr bytes.Buffer
  for i := 0; i < len(code); i++ {
    if code[i] > 9 {
      if code[i] == 10 {
        codeStr.WriteString("A")
      } else if code[i] == 11 {
        codeStr.WriteString("B")
      } else if code[i] == 12 {
        codeStr.WriteString("C")
      } else {
        codeStr.WriteString("D")
      }
    } else {
      fmt.Println(code[i])
      number := strconv.Itoa(code[i])
      codeStr.WriteString(number)
    }
  }
  return codeStr.String()
}

func move(direction string, currentPosition int) int {
  if direction == "U" {
    return up(currentPosition)
  } else if direction == "D" {
    return down(currentPosition)
  } else if direction == "L" {
    return left(currentPosition)
  } else if direction == "R" {
    return right(currentPosition)
  }
  return -1
}

func left(currentPosition int) int {
  if (currentPosition == 1 || currentPosition == 2 || currentPosition == 5 || currentPosition == 10 || currentPosition == 13) {
    return currentPosition
  } else {
    return currentPosition - 1
  }
}

func right(currentPosition int) int {
  if (currentPosition == 1 || currentPosition == 4 || currentPosition == 9 || currentPosition == 12 || currentPosition == 13) {
    return currentPosition
  } else {
    return currentPosition + 1
  }
}

func up(currentPosition int) int {
  if (currentPosition == 1 || currentPosition == 2 || currentPosition == 4 ||  currentPosition == 5 || currentPosition == 9) {
    return currentPosition
  } else if currentPosition == 13 || currentPosition == 3 {
    return currentPosition - 2
  } else {
    return currentPosition - 4
  }
}

func down(currentPosition int) int {
  if (currentPosition == 5 || currentPosition == 9|| currentPosition == 10 || currentPosition == 12 || currentPosition == 13) {
    return currentPosition
  } else if currentPosition == 1 || currentPosition == 11 {
    return currentPosition + 2
  } else {
    return currentPosition + 4
  }
}
