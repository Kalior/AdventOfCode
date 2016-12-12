package main

import (
  "fmt"
  "io/ioutil"
  "strconv"
  "regexp"
  "strings"
)

type Instruction interface {
  eval(curr int, registers map[string]int) int
}

type CpyValue struct {
  val int
  reg string
}

func (i CpyValue) eval(curr int, registers map[string]int) int {
  registers[i.reg] = i.val
  curr++
  return curr
}

type CpyReg struct {
  reg1, reg2 string
}

func (i CpyReg) eval(curr int, registers map[string]int) int {
  registers[i.reg2] = registers[i.reg1]
  curr++
  return curr
}

type Jnz struct {
  reg string
  dist int
}

func (i Jnz) eval(curr int, registers map[string]int) int {
  if registers[i.reg] != 0 {
    curr += i.dist
    return curr
  } else {
    curr++
    return curr
  }
}

type JnzVal struct {
  val, dist int
}

func (i JnzVal) eval(curr int, registers map[string]int) int {
  if i.val != 0 {
    curr += i.dist
    return curr
  } else {
    curr++
    return curr
  }
}

type Inc struct {
  reg string
}

func (i Inc) eval(curr int, registers map[string]int) int {
  registers[i.reg]++
  curr++
  return curr
}

type Dec struct {
  reg string
}

func (i Dec) eval(curr int, registers map[string]int) int {
  registers[i.reg]--
  curr++
  return curr
}

func main() {
  instructions := parseInput("day12input")
  interpret(instructions)
}

func parseInput(filename string) []Instruction {
  var instructions []Instruction
  input, err := ioutil.ReadFile(filename)
  r, _ := regexp.Compile("([a-z]+) ([0-9]+|[a-d]) ?(-?[0-9]+|[a-d])?")
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine) - 1; i++ {
      items := r.FindStringSubmatch(inputLine[i])
      fmt.Println(items)
      if items[1] == "cpy" {
        val, err := strconv.Atoi(items[2])
        if err == nil {
          instructions = append(instructions, CpyValue{val, items[3]})
        } else {
          instructions = append(instructions, CpyReg{items[2], items[3]})
        }
      } else if items[1] == "inc" {
        instructions = append(instructions, Inc{items[2]})
      } else if items[1] == "dec" {
        instructions = append(instructions, Dec{items[2]})
      } else {
        dist, _ := strconv.Atoi(items[3])
        val, err := strconv.Atoi(items[2])
        if err == nil {
          instructions = append(instructions, JnzVal{val, dist})
        } else {
          instructions = append(instructions, Jnz{items[2], dist})
        }
      }
    }
  }
  fmt.Println(instructions)
  return instructions
}

func interpret(instructions []Instruction) {
  registers := make(map[string]int)
  registers["a"] = 0
  registers["b"] = 0
  registers["c"] = 1
  registers["d"] = 0
  curr := 0
  for curr < len(instructions) {
    curr = instructions[curr].eval(curr, registers)
  }
  fmt.Println(registers)
}
