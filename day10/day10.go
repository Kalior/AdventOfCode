package main

import (
  "fmt"
  "io/ioutil"
  "strconv"
  "regexp"
  "strings"
)

type Instruction interface {
  performInstruction()
}

type GiveInstruction struct {
  bot int
  value int
}

type BotGiveInstruction struct {
  lowEntity, highEntity Entity
}

type Entity interface {
  setValue(i int)
  doWork()
}

type BotEntity struct {
  values [2]int
  instruction BotGiveInstruction
}

type OutPutEntity struct {
  value int
}


func main() {
  fmt.Println(parseInput("day10input"))
}

func parseInput(filename string) (int, int) {
  input, err := ioutil.ReadFile(filename)
  var outputs [21]OutPutEntity
  var bots [210]BotEntity
  for i := 0; i < len(bots); i++ {
    bots[i].values = [2]int{-1, -1}
  }
  rb, _ := regexp.Compile("bot ([0-9]+) gives low to (bot|output) ([0-9]+) and high to (bot|output) ([0-9]+)")
  rg, _ := regexp.Compile("value ([0-9]+) goes to bot ([0-9]+)")
  var giveInstructions []GiveInstruction
  if err != nil {
    fmt.Println(err)
  } else {
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine) - 1; i++ {
      botInstructions := rb.FindStringSubmatch(inputLine[i])
      if botInstructions != nil {
        botNr, _ := strconv.Atoi(botInstructions[1])
        lowNr, _ := strconv.Atoi(botInstructions[3])
        highNr, _ := strconv.Atoi(botInstructions[5])
        var bgi BotGiveInstruction
        if botInstructions[2] == "bot" {
          bgi.lowEntity = &bots[lowNr]
        } else {
          bgi.lowEntity = &outputs[lowNr]
        }
        if botInstructions[4] == "bot" {
          bgi.highEntity = &bots[highNr]
        } else {
          bgi.highEntity = &outputs[highNr]
        }
        bots[botNr].instruction = bgi
      } else {
        giveRegexp := rg.FindStringSubmatch(inputLine[i])
        value, _ := strconv.Atoi(giveRegexp[1])
        botNr, _ := strconv.Atoi(giveRegexp[2])
        g := GiveInstruction{botNr, value}
        giveInstructions = append(giveInstructions, g)
      }
    }
  }

  for _, v := range giveInstructions {
    v.performInstruction()
  }
  correctBot := -1
  for k, v := range bots {
    if (v.values[0] == 61 && v.values[1] == 17) || (v.values[1] == 61 && v.values[0] == 17) {
      correctBot = k
    }
  }

  return outputs[0].value * outputs[1].value * outputs[2].value, correctBot
}

func (b *BotEntity) doWork() {
  if b.values[0] != -1 && b.values[1] != -1 {
    min := Min(b.values[0], b.values[1])
    max := Max(b.values[0], b.values[1])
    b.instruction.highEntity.setValue(max)
    b.instruction.lowEntity.setValue(min)
    b.instruction.highEntity.doWork()
    b.instruction.lowEntity.doWork()
  }
}

func (o *OutPutEntity) doWork() {
  return
}

func (g *GiveInstruction) performInstruction(bot BotEntity) {
  bot.setValue(g.value)
  bot.doWork()
}

func (b *BotEntity) setValue(i int) {
  if b.values[0] == -1 {
    b.values[0] = i
  } else if b.values[1] == -1 {
    b.values[1] = i
  }
}

func (o *OutPutEntity) setValue(i int) {
  o.value = i
}

func Min(x, y int) int {
    if x < y {
        return x
    }
    return y
}

func Max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
