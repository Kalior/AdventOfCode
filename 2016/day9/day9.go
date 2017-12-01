package main

import (
  "fmt"
  "io/ioutil"
  "strconv"
  "bytes"
  "regexp"
  "strings"
)

func main() {
  compressed := parseInput("day9input")
  decompressed := decompress(compressed)
  fmt.Println("Compressed length ", len(compressed))
  fmt.Println("Decompressed length 1 ", count(decompressed))
  fmt.Println("Decompressed length 2 ", countDec(compressed))
}

func parseInput(filename string) string {
  day9input, err := ioutil.ReadFile(filename)
  compressed := ""
  if err != nil {
    fmt.Println(err)
  } else {
    day9inputLine := strings.Split(string(day9input), "\n")
    compressed = string(day9inputLine[0])
  }
  return compressed
}

func decompress(input string) string {
  var data bytes.Buffer
  r, _ := regexp.Compile("([0-9]+x[0-9]+)")
  rn, _ := regexp.Compile("[0-9]+")
  for input != "" {
    matchIndex := r.FindStringIndex(input)
    if matchIndex != nil && matchIndex[0] == 1 {
      match := r.FindString(input)
      numbers := rn.FindAllString(match, 2)
      length, _ := strconv.Atoi(numbers[0])
      times, _ := strconv.Atoi(numbers[1])
      for j := 0; j < times; j++ {
        data.WriteString(input[matchIndex[1]+1:length+matchIndex[1]+1])
      }
      input = input[matchIndex[1] + 1 + length:]
    } else {
      data.WriteByte(input[0])
      input = input[1:]
    }
  }
  return data.String()
}

func count(input string) int {
  count := 0
  for i := 0; i < len(input); i++ {
    if (string(input[i]) != " ") {
      count++
    }
  }
  return count
}

func countDec(input string) int {
  count := 0
  r, _ := regexp.Compile("([0-9]+x[0-9]+)")
  rn, _ := regexp.Compile("[0-9]+")
  for input != "" {
    matchIndex := r.FindStringIndex(input)
    if matchIndex != nil && matchIndex[0] == 1 {
      match := r.FindString(input)
      numbers := rn.FindAllString(match, 2)
      length, _ := strconv.Atoi(numbers[0])
      times, _ := strconv.Atoi(numbers[1])
      adddedValue := countDec(input[matchIndex[1]+1:length+matchIndex[1]+1]);
      count += adddedValue * times;
      //count += length * times
      input = input[matchIndex[1] + 1 + length:]
    } else {
      count++
      input = input[1:]
    }
  }
  return count
}
