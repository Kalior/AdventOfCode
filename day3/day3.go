package main

import (
  "fmt"
  "io/ioutil"
  "strings"
  "strconv"
)

func main() {
  day3input, err := ioutil.ReadFile("day3input")
  if err != nil {
    fmt.Println("Invalid input")
  }
  triangles := strings.Split(string(day3input), "\n")
  var trianglesArray [][3]int
  for i := 0; i < len(triangles) - 1; i++ {
    parseTriangle := strings.Split(strings.Trim(triangles[i], " "), " ")
    var triangle [3]string
    j := 0
    for i := 0; i < len(parseTriangle); i++ {
      if parseTriangle[i] != "" {
        triangle[j] = parseTriangle[i]
        j++
      }
    }
    var triangleArray [3]int
    for j := 0; j < 3; j++ {
      side, err := strconv.Atoi(strings.Trim(triangle[j], " "))
      if err != nil {
        fmt.Println(err)

      }
      triangleArray[j] = side
    }
    trianglesArray = append(trianglesArray, triangleArray)
  }
  fmt.Println((trianglesArray))
  fmt.Println(countPossibleTriangles(trianglesArray))
}

func countPossibleTriangles(triangles [][3]int) int {
  var validTriangles int
  for i := 0; i < len(triangles); i += 3{
    var triangleArray [3][3]int
    for j := 0; j < 3; j++ {
      triangleArray[j][0] = triangles[i][j]
      triangleArray[j][1] = triangles[i+1][j]
      triangleArray[j][2] = triangles[i+2][j]
    }
    validTriangles += testTriange(triangleArray[0])
    validTriangles += testTriange(triangleArray[1])
    validTriangles += testTriange(triangleArray[2])
  }
  return validTriangles
}

func testTriange(triangle [3]int) int {
  if triangle[0] >= triangle[1] && triangle[0] >= triangle[2] {
    return compareSides(triangle[0], triangle[1], triangle[2])
  } else if triangle[1] >= triangle[0] && triangle[1] >= triangle[2] {
    return compareSides(triangle[1], triangle[0], triangle[2])
  } else {
    return compareSides(triangle[2], triangle[1], triangle[0])
  }
}

func compareSides(largestSide int, side1 int, side2 int) int {
  if side1 + side2 > largestSide {
    return 1
  } else {
    return 0
  }
}
