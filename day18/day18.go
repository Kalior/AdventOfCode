package main

import (
  "fmt"
  "bytes"
)

func genRow(prevRow string) string {
  var row bytes.Buffer
  prevRow = "." + prevRow + "."

  for i := 1; i < len(prevRow)-1; i++ {
    if prevRow[i-1] == '^' && prevRow[i] == '^' && prevRow[i+1] == '.' {
      row.WriteString("^")
    } else if prevRow[i-1] == '.' && prevRow[i] == '^' && prevRow[i+1] == '^' {
      row.WriteString("^")
    } else if prevRow[i-1] == '^' && prevRow[i] == '.' && prevRow[i+1] == '.' {
      row.WriteString("^")
    } else if prevRow[i-1] == '.' && prevRow[i] == '.' && prevRow[i+1] == '^' {
      row.WriteString("^")
    } else {
      row.WriteString(".")
    }
  }
  return row.String()
}

func main() {
  var rows []string
  rows = append(rows, "^^^^......^...^..^....^^^.^^^.^.^^^^^^..^...^^...^^^.^^....^..^^^.^.^^...^.^...^^.^^^.^^^^.^^.^..^.^")
  for i := 1; i < 400000; i++ {
    rows = append(rows, genRow(rows[i-1]))
  }
  fmt.Println(countSafe(rows))
}

func countSafe(rows []string) int {
  count := 0
  for _, row := range rows {
    for j := 0; j < len(row); j++ {
      if row[j] == '.' {
        count++
      }
    }
  }
  return count
}
