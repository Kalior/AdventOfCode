package main

import(
  "fmt"
  "encoding/hex"
  "crypto/md5"
  "strings"
  "bytes"
)

type Position struct {
  x,y int
}

type ShortestPath struct {
  length int
  path string
}

func walk(soFar string, currentPos Position, p *ShortestPath) {
  if currentPos.x == 4 && currentPos.y == 4 {
    if len(soFar) > p.length {
      p.length = len(soFar)
      p.path = soFar
    }
  } else if len(soFar) < 1000 {
    var buffer bytes.Buffer
    buffer.WriteString("edjrjqaa")
    buffer.WriteString(soFar)
    hash := md5.Sum([]byte(buffer.String()))
    hashString := hex.EncodeToString(hash[:])
    if currentPos.y != 1 && strings.Contains("bcdef", string(hashString[0])) {
      soFarMove := soFar + "U"
      walk(soFarMove, Position{currentPos.x, currentPos.y-1}, p)
    }
    if currentPos.y != 4 && strings.Contains("bcdef", string(hashString[1])) {
      soFarMove := soFar + "D"
      walk(soFarMove, Position{currentPos.x, currentPos.y+1}, p)
    }
    if currentPos.x != 1 && strings.Contains("bcdef", string(hashString[2])) {
      soFarMove := soFar + "L"
      walk(soFarMove, Position{currentPos.x-1, currentPos.y}, p)
    }
    if currentPos.x != 4 && strings.Contains("bcdef", string(hashString[3])) {
      soFarMove := soFar + "R"
      walk(soFarMove, Position{currentPos.x+1, currentPos.y}, p)
    }
  }
}

func main() {
  p := ShortestPath{0, ""}
  walk("", Position{1,1}, &p)
  fmt.Println(p)
}
