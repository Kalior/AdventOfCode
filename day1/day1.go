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

func main() {
  var countVal = count()
  fmt.Println(countVal)
}


func count() int {
  var currentDirection = NORTH
  var upDown = 0
  leftRight := 0
  directions := []string{"L3", "R1", "L4", "L1", "L2", "R4", "L3", "L3", "R2", "R3", "L5", "R1", "R3", "L4", "L1", "L2", "R2", "R1", "L4", "L4", "R2", "L5", "R3", "R2", "R1", "L1", "L2", "R2", "R2", "L1", "L1", "R2", "R1", "L3", "L5", "R4", "L3", "R3", "R3", "L5", "L190", "L4", "R4", "R51", "L4", "R5", "R5", "R2", "L1", "L3", "R1", "R4", "L3", "R1", "R3", "L5", "L4", "R2", "R5", "R2", "L1", "L5", "L1", "L1", "R78", "L3", "R2", "L3", "R5", "L2", "R2", "R4", "L1", "L4", "R1", "R185", "R3", "L4", "L1", "L1", "L3", "R4", "L4", "L1", "R5", "L5", "L1", "R5", "L1", "R2", "L5", "L2", "R4", "R3", "L2", "R3", "R1", "L3", "L5", "L4", "R3", "L2", "L4", "L5", "L4", "R1", "L1", "R5", "L2", "R4", "R2", "R3", "L1", "L1", "L4", "L3", "R4", "L3", "L5", "R2", "L5", "L1", "L1", "R2", "R3", "L5", "L3", "L2", "L1", "L4", "R4", "R4", "L2", "R3", "R1", "L2", "R1", "L2", "L2", "R3", "R3", "L1", "R4", "L5", "L3", "R4", "R4", "R1", "L2", "L5", "L3", "R1", "R4", "L2", "R5", "R4", "R2", "L5", "L3", "R4", "R1", "L1", "R5", "L3", "R1", "R5", "L2", "R1", "L5", "L2", "R2", "L2", "L3", "R3", "R3", "R1"}
  for i := 0; i < len(directions); i++ {
    length, err := strconv.Atoi(directions[i][1:])
    fmt.Println(err)
    if (string(directions[i][0]) == string("L")) {
      if (currentDirection == NORTH) {
        currentDirection = WEST
        leftRight -= length;
      } else if (currentDirection == SOUTH) {
        currentDirection = EAST
        leftRight += length;
      } else if (currentDirection == WEST) {
        currentDirection = SOUTH
        upDown -= length;
      } else if (currentDirection == EAST) {
        currentDirection = NORTH
        upDown += length;
      }
    } else if (string(directions[i][0]) == string("R")) {
      if (currentDirection == NORTH) {
        currentDirection = EAST
        leftRight += length;
      } else if (currentDirection == SOUTH) {
        currentDirection = WEST
        leftRight -= length;
      } else if (currentDirection == WEST) {
        currentDirection = NORTH
        upDown += length;
      } else if (currentDirection == EAST) {
        currentDirection = SOUTH
        upDown -= length;
      }
    }
  }
  if (upDown < 0) {
    upDown = -upDown
  }
  if (leftRight < 0) {
    leftRight = -leftRight
  }
  var returnValue = upDown + leftRight
  return returnValue
}
