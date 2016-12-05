package main

import (
  "fmt"
  "io/ioutil"
  "strings"
  "strconv"
  "bytes"
)

func main() {
  rooms := parseRoomInput("day4input")
  validRooms := checkRooms(rooms)
  fmt.Println(validRooms)
}

func parseRoomInput(filename string) [][]string {
  day4input, err := ioutil.ReadFile(filename)
  var rooms [][]string
  if err != nil {
    fmt.Println(err)
  }
  day4inputLine := strings.Split(string(day4input), "\n")
  for i := 0; i < len(day4inputLine) - 1; i++ {
    room := strings.Split(day4inputLine[i], "-")
    rooms = append(rooms, room)
  }
  return rooms
}

func checkRooms(rooms [][]string) int {
  value := 0
  var roomNames []string
  var validRooms [][]string
  for i := 0; i < len(rooms); i++ {
    occurancesMap := make(map[string]int)
    for j := 0; j < len(rooms[i]) - 1; j++ {
      for k := 0; k < len(rooms[i][j]); k++ {
        occurancesMap[string(rooms[i][j][k])] += 1
      }
    }
    idCheckSum := strings.Split(string(rooms[i][len(rooms[i]) - 1]), "[")
    checkSum := idCheckSum[1][:len(idCheckSum[1]) - 1]
    if checkRoom(occurancesMap, checkSum) {
      validRooms = append(validRooms, rooms[i])
      v, err := strconv.Atoi(idCheckSum[0])
      if err != nil {
        fmt.Println(err)
      }
      value += v
      name := decrytName(v, rooms[i])
      roomNames = append(roomNames, name)
      if strings.Contains(name, "northpole object") {
        fmt.Println(name, v)
      }
    }
  }
  return value
}

func checkRoom(occurancesMap map[string]int, checkSum string) bool {
  for i := 0; i < len(checkSum); i++ {
    if !placeI(i, occurancesMap, string(checkSum[i])) {
      return false
    }
  }
  return true
}

func placeI(i int, occurancesMap map[string]int, character string) bool {
  value := occurancesMap[character]
  var larger int
  for k,v := range occurancesMap {
    if k != character && v > value {
      larger++
    }
  }
  return i >= larger
}

func decrytName (id int, room []string) string {
  shifts := id % 26
  var name bytes.Buffer
  for i := 0; i < len(room) - 1; i++ {
    for j := 0; j < len(room[i]); j++ {
      letter := rune(room[i][j])
      if letter >= 'a' && letter <= 'z' - rune(shifts) {
        name.WriteString(string(letter + rune(shifts)))
      } else {
        name.WriteString(string(letter + rune(shifts) - 26))
      }
    }
    name.WriteString(" ")
  }
  return name.String()
}
