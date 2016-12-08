package main

import (
  "fmt"
  "io/ioutil"
  "strings"
)

type IPv7 struct {
  supernet []string
  hypernets []string
}

func main() {
  fmt.Println(filterABAs(parseData("day7input")))

}

func parseData(filename string) []IPv7 {
  var ips []IPv7
  day7data, err := ioutil.ReadFile(filename)
  if err != nil {
    fmt.Println(err)
  } else {
    day7dataLines := strings.Split(string(day7data), "\n")
    for i := 0; i < len(day7dataLines) - 1 ; i++ {
      var str []string
      var hypernets []string
      split1 := strings.Split(day7dataLines[i], "[")
      str = append(str, split1[0])
      for i := 0; i < len(split1) - 1; i++ {
        split2 := strings.Split(split1[i+1], "]")
        hypernets = append(hypernets, split2[0])
        str = append(str, split2[1])
      }
      ip := IPv7{str, hypernets}
      ips = append(ips, ip)
    }
  }
  return ips
}

func filterABBAs(ips []IPv7) int {
  supportCounter := 0
  for i := 0; i < len(ips); i++ {
      if (hasAtLeastABBA(ips[i].supernet) && !hasAtLeastABBA(ips[i].hypernets)) {
      supportCounter++
    }
  }
  return supportCounter
}

func hasAtLeastABBA(datas []string) bool {
  for i := 0; i < len(datas); i++ {
    if hasABBA(datas[i]) {
      return true
    }
  }
  return false
}

func hasABBA(data string) bool {
  for i := 0; i < len(data)-3; i++ {
    if data[i] == data[i+3] && data[i+1] == data[i+2] && data[i] != data[i+1] {
      return true
    }
  }
  return false
}

func filterABAs(ips []IPv7) int {
  supportCounter := 0
  for i := 0; i < len(ips); i++ {
    abas := findAbas(ips[i].supernet)
    if (hasAtLeastCorrABA(ips[i].hypernets, abas)) {
      supportCounter++
    }
  }
  return supportCounter
}

func findAbas(datas []string) [][2]byte {
  var allAbas [][2]byte
  for i := 0; i < len(datas); i++ {
    b, abas := hasABA(datas[i])
    if b {
      allAbas = append(allAbas, abas...)
    }
  }
  return allAbas
}

func hasABA(data string) (bool, [][2]byte) {
  var abas [][2]byte
  for i := 0; i < len(data)-2; i++ {
    if data[i] == data[i+2] && data[i] != data[i+1] {
      abas = append(abas, [2]byte{data[i], data[i+1]})
    }
  }
  if len(abas) == 0 {
    return false, nil
  } else {
    return true, abas
  }
}

func hasAtLeastCorrABA(datas []string, abas [][2]byte) bool {
  for j := 0; j < len(abas); j++ {
    for i := 0; i < len(datas); i++ {
      if hasCorrABA(datas[i], abas[j]) {
        return true
      }
    }
  }
  return false
}

func hasCorrABA(data string, corr [2]byte) bool {
  for i := 0; i < len(data) - 2; i++ {
    if data[i] == corr[1] && data[i+1] == corr[0] && data[i+2] == corr[1] {
      return true
    }
  }
  return false
}
