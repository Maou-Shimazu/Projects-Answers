package main

import (
	"fmt"
)

func main() {
	fmt.Println("Please enter the string you want checked...")
  var input string
  fmt.Scan(&input)
  var rev string
  for i := len(input) - 1; i >= 0; i-- {
    rev += string(input[i])
  }
  if input == rev {
    fmt.Printf("%v is a palindrome\n", input)
  } else {
    fmt.Printf("%v is not a palindrome\n", input)
  }
}
