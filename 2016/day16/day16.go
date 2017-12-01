package main

import (
  "fmt"
  "bytes"
)

func data(a string) string {
  var buffer bytes.Buffer
  n := len(a)
  for i := 0; i < n; i++ {
    buffer.WriteString(string(a[n-1-i]))
  }
  b := buffer.String()
  var modB bytes.Buffer
  for i := 0; i < len(b); i++ {
    if (b[i] == '1') {
      modB.WriteString("0")
    } else {
      modB.WriteString("1")
    }
  }
  b = modB.String()
  var final bytes.Buffer
  final.WriteString(a)
  final.WriteString("0")
  final.WriteString(b)
  return final.String()
}

func allData(length int) string {
  d := "11011110011011101"
  for len(d) < length {
    d = data(d)
  }

  return d[:length]
}

func main() {
  fmt.Println("DISK ONE")
  d := allData(272)
  sum := checkSum(d)
  fmt.Println(sum)
  fmt.Println("DISK TWO")
  d2 := allData(35651584)
  sum2 := checkSum(d2)
  fmt.Println(sum2)
}

func checkSum(d string) string {
  checkSum := Sum(d)
  for len(checkSum) % 2 == 0 {
    checkSum = Sum(checkSum)
  }
  return checkSum
}

func Sum(d string) string {
  var sumBuffer bytes.Buffer
  for i := 0; i < len(d) - 1; i+= 2 {
    if d[i] == d[i+1] {
      sumBuffer.WriteString("1")
    } else {
      sumBuffer.WriteString("0")
    }
  }
  ret := sumBuffer.String()
  return ret
}
