package main

import (
	"fmt"
  "strings"
  "strconv"
)

func main() {
  var sum int
	fmt.Println("Enter the card number: ")
  var card string
  fmt.Scan(&card)
  arr := strings.Split(card, "")
  for i := range arr {
    num, err := strconv.Atoi(arr[i])
    if err != nil {
      println("Please enter numbers only")
    }
    if num * 2 > 9 {
      sum += num * 2 - 9
    } else {
      sum += num * 2
    }
  }
  if sum / 10 == 0 {
    println("Valid card number")
  } else {
    println("Invalid card number")
  }
}
