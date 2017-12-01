package main

import (
  "fmt"
  "strconv"
)

type Direction int
const (
  NORTH Direction = iota
  SOUTH
  WEST
  EAST
)

type Position struct {
    x, y int
}


func main() {
  var countVal = count()
  fmt.Println(countVal)
}


func count() int {
  visitedMap := make(map[Position]bool)
  var currentDirection = NORTH
  var upDown = 0
  var leftRight = 0
  directions := []string{"L3", "R1", "L4", "L1", "L2", "R4", "L3", "L3", "R2", "R3", "L5", "R1", "R3", "L4", "L1", "L2", "R2", "R1", "L4", "L4", "R2", "L5", "R3", "R2", "R1", "L1", "L2", "R2", "R2", "L1", "L1", "R2", "R1", "L3", "L5", "R4", "L3", "R3", "R3", "L5", "L190", "L4", "R4", "R51", "L4", "R5", "R5", "R2", "L1", "L3", "R1", "R4", "L3", "R1", "R3", "L5", "L4", "R2", "R5", "R2", "L1", "L5", "L1", "L1", "R78", "L3", "R2", "L3", "R5", "L2", "R2", "R4", "L1", "L4", "R1", "R185", "R3", "L4", "L1", "L1", "L3", "R4", "L4", "L1", "R5", "L5", "L1", "R5", "L1", "R2", "L5", "L2", "R4", "R3", "L2", "R3", "R1", "L3", "L5", "L4", "R3", "L2", "L4", "L5", "L4", "R1", "L1", "R5", "L2", "R4", "R2", "R3", "L1", "L1", "L4", "L3", "R4", "L3", "L5", "R2", "L5", "L1", "L1", "R2", "R3", "L5", "L3", "L2", "L1", "L4", "R4", "R4", "L2", "R3", "R1", "L2", "R1", "L2", "L2", "R3", "R3", "L1", "R4", "L5", "L3", "R4", "R4", "R1", "L2", "L5", "L3", "R1", "R4", "L2", "R5", "R4", "R2", "L5", "L3", "R4", "R1", "L1", "R5", "L3", "R1", "R5", "L2", "R1", "L5", "L2", "R2", "L2", "L3", "R3", "R3", "R1"}
  //directions := []string{"R8", "R4", "R4", "R8"}
  for i := 0; i < len(directions); i++ {
    length, err := strconv.Atoi(directions[i][1:])
    if (err != nil) {
      return -1
    }
    upDownChange := 0
    leftRightChange := 0
    if (string(directions[i][0]) == string("L")) {
      if (currentDirection == NORTH) {
        currentDirection = WEST
        leftRightChange -= length;
      } else if (currentDirection == SOUTH) {
        currentDirection = EAST
        leftRightChange += length;
      } else if (currentDirection == WEST) {
        currentDirection = SOUTH
        upDownChange -= length;
      } else if (currentDirection == EAST) {
        currentDirection = NORTH
        upDownChange += length;
      }
    } else {
      if (currentDirection == NORTH) {
        currentDirection = EAST
        leftRightChange += length;
      } else if (currentDirection == SOUTH) {
        currentDirection = WEST
        leftRightChange -= length;
      } else if (currentDirection == WEST) {
        currentDirection = NORTH
        upDownChange += length;
      } else if (currentDirection == EAST) {
        currentDirection = SOUTH
        upDownChange -= length;
      }
    }
    if (upDownChange < 0) {
      for i := 0; i > upDownChange; i-- {
        mapValue := checkMap(visitedMap, upDown + i, leftRight)
        if mapValue != -1 {
          return mapValue
        }
      }
    } else {
      for i := 0; i < upDownChange; i++ {
        mapValue := checkMap(visitedMap, upDown + i, leftRight)
        if mapValue != -1 {
          return mapValue
        }
      }
    }
    if (leftRightChange < 0) {
      for j := 0; j > leftRightChange; j-- {
        mapValue := checkMap(visitedMap, upDown, leftRight + j)
        if mapValue != -1 {
          return mapValue
        }
      }
    } else {
      for j := 0; j < leftRightChange; j++ {
        mapValue := checkMap(visitedMap, upDown, leftRight + j)
        if mapValue != -1 {
          return mapValue
        }
      }
    }
    leftRight += leftRightChange
    upDown += upDownChange
  }
  fmt.Println("None found")
  return -1
}

func checkMap(visitedMap map[Position]bool, upDown int, leftRight int) int {
  visited := visitedMap[Position{leftRight, upDown}]
  if visited {
    if (upDown < 0) {
      upDown = -upDown
    }
    if (leftRight < 0) {
      leftRight = -leftRight
    }
    return upDown + leftRight
  }
  visitedMap[Position{leftRight, upDown}] = true
  return -1
}
