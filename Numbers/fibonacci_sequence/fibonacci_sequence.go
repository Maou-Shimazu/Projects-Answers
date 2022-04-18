package main

import (
	"fmt"
)

func main() {
	fmt.Println("Enter the number you want calculated up to...")
  var loc int
  fmt.Scan(&loc)
  sequence := []int{0, 1}
  for i := 1; i < loc + 1; i++ {
    sum := sequence[i] + sequence[i-1]
    sequence = append(sequence, sum)
  }
  fmt.Println(sequence)
}
