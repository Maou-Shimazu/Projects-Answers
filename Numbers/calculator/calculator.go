package main

import (
	"fmt"
)

func main() {
	fmt.Println("Please enter your first number...")
	var fnum float64
	fmt.Scan(&fnum)
	fmt.Println("Please enter your operator...")
	var op string
	fmt.Scan(&op)
	fmt.Println("Please enter your second number...")
	var snum float64
	fmt.Scan(&snum)
	if op == "+" {
		fmt.Println(fnum + snum)
	} else if op == "-" {
		fmt.Println(fnum - snum)
	} else if op == "*" {
		fmt.Println(fnum * snum)
	} else if op == "/" {
		fmt.Println(fnum / snum)
	} else {
		fmt.Println("[ERROR] Invalid operator: " + op)
	}
}
