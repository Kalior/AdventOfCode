package main

import (
  "fmt"
  "strings"
  "regexp"
  "io/ioutil"
  "strconv"
)

type Node struct {
  x, y, size, used, avail, useperc int
}

type Position struct {
  x,y int
}

func parseInput(filename string) []Node {
  var nodes []Node
  input, err := ioutil.ReadFile(filename)
  if err != nil {
    fmt.Println(err)
  } else {
    rt := regexp.MustCompile("([0-9]+)")
    inputLine := strings.Split(string(input), "\n")
    for i := 2; i < len(inputLine)-1; i++ {
      t := rt.FindAllString(inputLine[i], -1)
      x, _ := strconv.Atoi(t[0])
      y, _ := strconv.Atoi(t[1])
      size, _ := strconv.Atoi(t[2])
      used, _ := strconv.Atoi(t[3])
      avail, _ := strconv.Atoi(t[4])
      perc, _ := strconv.Atoi(t[5])
      nodes = append(nodes, Node{x,y,size,used,avail,perc})
    }
  }
  return nodes
}

func main() {
  nodes := parseInput("day22input")
  viablePairs := viablePairs(nodes)
  fmt.Println("Task 1: ", len(viablePairs))
  solveGood(nodes)
}

func viablePairs(nodes []Node) [][2]Node{
  var viablePairs [][2]Node
  for i := 0; i < len(nodes); i++ {
    if nodes[i].used != 0 {
      for j := 0; j < len(nodes); j++ {
        if i != j && nodes[i].used <= nodes[j].avail {
          viablePairs = append(viablePairs, [2]Node{nodes[i],nodes[j]})
        }
      }
    }
  }
  return viablePairs
}

func abs(x, y int) int {
  if x-y < 0 {
    return -(x-y)
  } else {
    return x-y
  }
}

func solveGood(nodes []Node) {
  viableP := viablePairs(nodes)
  steps, nextNodes := step(Position{17, 22}, Position{36, 0}, nodes, Position{-1, -1}, 100, viableP)
  pos := Position{37, 0}
  for pos.x != 0 || pos.y != 0 {
    steps++
    s, newNodes, tryPos := tryPoses(nextNodes, pos, 6, viableP)
    if s == -1 {
      break
    }
    //fmt.Println(s, tryPos)
    steps += s
    nextNodes = newNodes
    pos = tryPos
  }
  // steps += 36*5
  fmt.Println("Task 2: ", steps)
}

func tryPoses(nextNodes []Node, pos Position, maxLength int, viableP [][2]Node) (int, []Node, Position) {
  dataPos := Position{pos.x-1, pos.y}
  tryNodes := dataStep(nextNodes, pos, dataPos)
  s, newNodes := step(pos, Position{pos.x-2, pos.y}, tryNodes, dataPos, maxLength, viableP)
  if s != 99999 && s != 0 && s != -1 {
    return s, newNodes, Position{pos.x-1, pos.y}
  } else {
    return -1, newNodes, Position{pos.x-2, pos.y}
  }
}

func dataStep(newNodes []Node, currPos, nextPos Position) []Node {
  node1 := -1
  node2 := -1
  for j := 0; j < len(newNodes); j++ {
    if newNodes[j].x == currPos.x && newNodes[j].y == currPos.y {
      node1 = j
    }
    if newNodes[j].x == nextPos.x && newNodes[j].y == nextPos.y {
      node2 = j
    }
  }
  if node1 != -1 && node2 != -1 {
    newNodes[node2].avail -= newNodes[node1].used
    newNodes[node1].used = 0
    newNodes[node1].avail = newNodes[node1].size
    return newNodes
  } else {
    fmt.Println("ASD")
    return newNodes
  }
}

func step(from Position, to Position, nodes []Node, dataPos Position, maxLength int, viableP [][2]Node) (int, []Node) {
  node := Node{-1,-1,-1,-1,-1,-1}
  for i := 0; i < len(nodes) && node.x == -1; i++ {
    if nodes[i].x == from.x && nodes[i].y == from.y {
      node = nodes[i]
    }
  }
  visited := make(map[Position]int)
  for i := 0; i < 38; i++ {
    for j := 0; j < 26; j++ {
      visited[Position{i,j}] = 99999
    }
  }
  finalNodes := make([]Node, len(nodes))
  stepRec(node, 0, nodes, &visited, to, &finalNodes, dataPos, maxLength, viableP)
  return visited[to], finalNodes
}

func stepRec(node Node, length int, nodes []Node, visited *map[Position]int, to Position, finalNodes *[]Node, dataPos Position, maxLength int, viableP [][2]Node) {
  nodePos := Position{node.x, node.y}
  if (*visited)[nodePos] > length && nodePos != dataPos && length <= maxLength {
    (*visited)[Position{node.x, node.y}] = length
    if to.x == node.x && to.y == node.y {
      (*finalNodes) = nodes
    } else {
      adjacentPairs := calcAdjacentPairs(node, viableP)
      length += 1
      if false {
        fmt.Println("Length ", length)
        fmt.Println("Node ", node)
        fmt.Println("Pairs ", adjacentPairs)
        fmt.Println("Number of pairs ",len(adjacentPairs))
      }
      for i := 0; i < len(adjacentPairs); i++ {
        newNodes := make([]Node, len(nodes))
        copy(newNodes, nodes)
        var newNode Node
        node1 := -1
        node2 := -1
        for j := 0; j < len(newNodes); j++ {
          if newNodes[j].x == node.x && newNodes[j].y == node.y {
            node2 = j
          }
          if newNodes[j].x == adjacentPairs[i].x && newNodes[j].y == adjacentPairs[i].y {
            node1 = j
          }
        }
        newNodes[node2].avail -= newNodes[node1].used
        newNodes[node1].used = 0
        newNodes[node1].avail = newNodes[node1].size
        newNode = newNodes[node1]
        stepRec(newNode, length, newNodes, visited, to, finalNodes, dataPos, maxLength, viableP)
      }
    }
  }
}

func calcAdjacentPairs(node Node, pairs [][2]Node) []Node {
  var adjacentPairs []Node
  for i := 0; i < len(pairs); i++ {
    if abs(node.x, pairs[i][0].x) == 1 && abs(node.y, pairs[i][0].y) == 0 ||
        abs(node.x, pairs[i][0].x) == 0 && abs(node.y, pairs[i][0].y) == 1 {
      adjacentPairs = append(adjacentPairs, pairs[i][0])
    }
  }
  return adjacentPairs
}
