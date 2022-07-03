package main

import (
	"fmt"
)

func main() {
	var number, third int
	var name string
	fmt.Print("Enter your name:")
	fmt.Scan(&name)
	fmt.Printf("Hello, %s, enter the number to proceed:", name)
	fmt.Scanf("%v", &number)
	first := 0
	second := 1
	fmt.Printf("The fibonacci sequence:\n%d, %d, ", first, second)
	for i := 2; i < number; i++ {
		third = first + second
		first = second
		second = third
		fmt.Printf("%d, ", third)
	}
}
