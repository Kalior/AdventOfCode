package main

import (
  "fmt"
  "io/ioutil"
  "strconv"
  "regexp"
  "strings"
)

type Instruction interface {
  eval(curr int, registers map[string]int, instructions *[]Instruction) int
}

type Cpy struct {
  val, reg string
}

func (i Cpy) eval(curr int, registers map[string]int, instructions *[]Instruction) int {
  val, err := strconv.Atoi(i.val)
  curr++
  if err == nil {
    registers[i.reg] = val
  } else {
    registers[i.reg] = registers[i.val]
  }
  return curr
}

type Jnz struct {
  val, dist string
}

func (i Jnz) eval(curr int, registers map[string]int, instructions *[]Instruction) int {
  val, errv := strconv.Atoi(i.val)
  dist, errd := strconv.Atoi(i.dist)
  if val != 0 && errv == nil && errd == nil {
    curr += dist
  } else if errv != nil && registers[i.val] != 0 && errd != nil {
    curr += registers[i.dist]
  } else if errv == nil && val != 0 && errd != nil {
    curr += registers[i.dist]
  } else if errv != nil && registers[i.val] != 0 && errd == nil {
    curr += dist
  } else {
    curr++
  }
  return curr
}

type Inc struct {
  reg string
}

func (i Inc) eval(curr int, registers map[string]int, instructions *[]Instruction) int {
  registers[i.reg]++
  curr++
  return curr
}

type Dec struct {
  reg string
}

func (i Dec) eval(curr int, registers map[string]int, instructions *[]Instruction) int {
  registers[i.reg]--
  curr++
  return curr
}

type Tgl struct {
  reg string
}

func (i Tgl) eval(curr int, registers map[string]int, instructions *[]Instruction) int {
  val, err := strconv.Atoi(i.reg)
  if err != nil {
    val = registers[i.reg]
  }
  if curr+val < len((*instructions)) {
    testInstruction := (*instructions)[curr + val]
    instructionInc, isInc := testInstruction.(Inc)
    instructionDec, isDec := testInstruction.(Dec)
    instructionTgl, isTgl := testInstruction.(Tgl)
    instructionCpy, isCpy := testInstruction.(Cpy)
    instructionJnz, isJnz := testInstruction.(Jnz)
    if isInc {
      (*instructions)[curr+val] = Dec{instructionInc.reg}
    } else if isTgl {
      (*instructions)[curr+val] = Inc{instructionTgl.reg}
    } else if isDec {
      (*instructions)[curr+val] = Inc{instructionDec.reg}
    } else if isJnz {
      (*instructions)[curr+val] = Cpy{instructionJnz.val, instructionJnz.dist}
    } else if isCpy {
      (*instructions)[curr+val] = Jnz{instructionCpy.val, instructionCpy.reg}
    }
  }
  curr++
  return curr
}

type TglReg struct {
  reg string
}

func main() {
  instructions := parseInput("day23input")
  interpret(instructions)
}

func parseInput(filename string) []Instruction {
  var instructions []Instruction
  input, err := ioutil.ReadFile(filename)
  r, _ := regexp.Compile("([a-z]+) (-?[0-9]+|[a-d]) ?(-?[0-9]+|[a-d])?")
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine) - 1; i++ {
      items := r.FindStringSubmatch(inputLine[i])
      if items[1] == "cpy" {
        instructions = append(instructions, Cpy{items[2], items[3]})
      } else if items[1] == "inc" {
        instructions = append(instructions, Inc{items[2]})
      } else if items[1] == "dec" {
        instructions = append(instructions, Dec{items[2]})
      } else if items[1] == "jnz" {
        instructions = append(instructions, Jnz{items[2], items[3]})
      } else {
        instructions = append(instructions, Tgl{items[2]})
      }
    }
  }
  return instructions
}

func interpret(instructions []Instruction) {
  registers := make(map[string]int)
  registers["a"] = 12
  registers["b"] = 0
  registers["c"] = 0
  registers["d"] = 0
  curr := 0
  for curr < len(instructions) {
    curr = instructions[curr].eval(curr, registers, &instructions)
  }
  fmt.Println(registers)
}
