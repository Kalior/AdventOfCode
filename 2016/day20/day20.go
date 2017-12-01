package main

import (
  "fmt"
  "io/ioutil"
  "strings"
  "strconv"
  "regexp"
  // "sort"
  "time"
)

func parseInput(filename string) [][2]int {
  var ranges [][2]int
  input, err := ioutil.ReadFile(filename)
  if err != nil {
    fmt.Println(err)
  } else {
    r, _ := regexp.Compile("([0-9]+)-([0-9]+)")
    inputLine := strings.Split(string(input), "\n")
    for i := 0; i < len(inputLine) - 1; i++ {
      items := r.FindStringSubmatch(inputLine[i])
      btm, _ := strconv.Atoi(items[1])
      high, _ := strconv.Atoi(items[2])
      ranges = append(ranges, [2]int{btm, high})
    }
  }
  return ranges
}

func main() {
  ranges := parseInput("day20input")
  t := time.Now()
  solvebetter(ranges)
  fmt.Println(time.Since(t))
}

func solve(ranges [][2]int) {
  allowed := 0
  for i := 0; i <= 4294967295; i++ {
    notInAnyRange := true
    for j := 0; j < len(ranges) && notInAnyRange; j++ {
      if i >= ranges[j][0] && i <= ranges[j][1] {
        notInAnyRange = false
      }
    }
    if notInAnyRange {
      allowed++
      fmt.Println(i)
    }
  }
  fmt.Println(allowed)
}

func solvebetter(ranges [][2]int) {
  firstIp := -1
  allowed := 0
  for i := 0; i < 4294967295; {
    notInAnyRange := true
    for j := 0; j < len(ranges) && notInAnyRange; j++ {
      if i >= ranges[j][0] && i <= ranges[j][1] {
        notInAnyRange = false
      }
    }
    if notInAnyRange {
      if firstIp == -1 {
        firstIp = i
      }
      allowed++
      i++
    } else {
      lowestRange := [2]int{4294967295, 0}
      for j := 0; j < len(ranges); j++ {
        if (ranges[j][0] < lowestRange[0] && i >= ranges[j][0] && i <= ranges[j][1] ) {
          lowestRange = ranges[j]
        }
      }
      i = lowestRange[1]+1
    }
  }
  fmt.Println("Task 1: ", firstIp)
  fmt.Println("Task 2: ", allowed)
}
