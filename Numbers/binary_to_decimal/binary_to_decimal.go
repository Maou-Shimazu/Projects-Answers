package main

import (
	"fmt"
  "strconv"
)

func main() {
	fmt.Println("Enter your binary number...")
  var input string
  fmt.Scan(&input)
  decimal, _ := strconv.ParseInt(input, 2, 64)  
  fmt.Println(decimal)
}
