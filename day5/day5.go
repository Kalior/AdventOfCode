package main

import (
  "fmt"
  "crypto/md5"
  "strconv"
  "encoding/hex"
  "strings"
  "time"
)

type Code struct {
  password [8]string
}

func main() {
  code := &Code{[8]string{"-", "-", "-", "-", "-", "-", "-", "-"}}
  t := time.Now()
  code.findHash("wtnhxymk")
  fmt.Println(time.Since(t))
}

func (c *Code) findHash(input string) {
  passwordCount := 0
  for i := 0; passwordCount < 8 ; i++ {
    numberByte := []byte(strconv.Itoa(i))
    data := append([]byte(input), numberByte...)
    hash := md5.Sum(data)
    hexHash := hex.EncodeToString(hash[:])
    if "00000" == hexHash[:5] {
      position, err := strconv.Atoi(string(hexHash[5]))
      if err == nil && position < 8 && c.password[position] == "-" {
        c.password[position] = string(hexHash[6])
        passwordCount++
        fmt.Println(strings.Join(c.password[:],""))
      }
    }
  }
}
