package main

import (
  "fmt"
  "io/ioutil"
  "strings"
  "bytes"
)

type RepeatedMessage struct {
  messages []string
  correctedMessage string
}

func main() {
  message := parseInput("day6input")
  fmt.Println(message)
  message.correctMessage()
  fmt.Println(message)
}

func parseInput(filename string) RepeatedMessage {
  day6input, err := ioutil.ReadFile(filename)
  if err != nil {
    fmt.Println(err)
  } else {
    day6inputLine := strings.Split(string(day6input), "\n")
    r := RepeatedMessage{make([]string, len(day6inputLine)), ""}
    r.messages = day6inputLine
    return r
  }
  return RepeatedMessage{make([] string, 1), ""}
}

func (r *RepeatedMessage) correctMessage() {
  var messageStr bytes.Buffer
  for i := 0; i < 8; i++ {
    letters := make(map[string]int)
    for j := 0; j < len(r.messages) - 1; j++ {
      letters[string(r.messages[j][i])]++
    }
    leastCommonLetter := ""
    leastCommonLetterCount := 999
    for k, v := range letters {
      if (v < leastCommonLetterCount) {
        leastCommonLetterCount = v
        leastCommonLetter = k
      }
    }
    messageStr.WriteString(leastCommonLetter)
  }
  r.correctedMessage = messageStr.String()
}
