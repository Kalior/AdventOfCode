package main

import (
  "fmt"
  "crypto/md5"
  "encoding/hex"
  "bytes"
  "time"
  "strings"
)

func main() {
  t := time.Now()
  generateKeys(false)
  fmt.Println(time.Since(t))
  t2 := time.Now()
  generateKeys(true)
  fmt.Println(time.Since(t2))
}

func generateKeys(stretch bool) {
  var keys []string
  computedHashes := make([]string, 1000000)
  for i := 0; 64 > len(keys); i++ {
    if computedHashes[i] == "" {
      computedHashes[i] = computeHex(i, stretch)
    }
    firstTest := false
    var letter byte
    for j := 0; j < len(computedHashes[i]) - 2 && !firstTest; j++ {
      if computedHashes[i][j] == computedHashes[i][j+1] && computedHashes[i][j] == computedHashes[i][j+2] {
        firstTest = true
        letter = computedHashes[i][j]
      }
    }
    secondTest := false
    if firstTest {
      for j := i+1; j < i + 1000 && !secondTest; j++ {
        if computedHashes[j] == "" {
          computedHashes[j] = computeHex(j, stretch)
        }
        secondTest = strings.Contains(computedHashes[j], strings.Repeat(string(letter), 5))
      }
    }
    if secondTest {
      keys = append(keys, computedHashes[i])
    }
    if len(keys) == 64 {
      fmt.Println(i)
      fmt.Println(len(keys))
    }
  }
}

func computeHex(i int, stretch bool) string {
  var buffer bytes.Buffer
  buffer.WriteString("cuanljph")
  hexI := fmt.Sprintf("%d", i)
  buffer.WriteString(hexI)
  hash := md5.Sum([]byte(buffer.String()))
  if stretch {
    for i := 0; i < 2016; i++ {
      hash = md5.Sum([]byte(hex.EncodeToString(hash[:])))
    }
  }
  return hex.EncodeToString(hash[:])
}
