package main

import (
  "fmt"
  "crypto/md5"
  "strconv"
  "encoding/hex"
  "strings"
)

func main() {
  findHash("wtnhxymk")
}

func findHash(input string) [8]string {
  password := [8]string{" ", " ", " ", " ", " ", " ", " ", " "}
  passwordCount := 0
  for i := 0; passwordCount < 8 ; i++ {
    numberByte := []byte(strconv.Itoa(i))
    data := append([]byte(input), numberByte...)
    hash := md5.Sum(data)
    hexHash := hex.EncodeToString(hash[:])
    if "00000" == hexHash[:5] {
      position, err := strconv.Atoi(string(hexHash[5]))
      if err == nil && position < 8 && password[position] == " " {
        password[position] = string(hexHash[6])
        passwordCount++
        fmt.Println(strings.Join(password[:],""))
      }
    }
  }
  return password
}
