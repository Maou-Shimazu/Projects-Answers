package main

import (
	"fmt"
	"math"
)

func Addition(a, b int) int {
	return a + b
}
func Substraction(a, b int) int {
	return a - b
}
func Multiplication(a, b int) int {
	return a * b
}
func Division(a, b float64) float64 {
	return a / b
}
func Exponentiation(a, b float64) float64 {
	return math.Pow(a, b)
}
func SquareRoot(a, b float64) float64 {
	return math.Pow(a, 1/b)
}
func Log10(a float64) float64 {
	return math.Log10(a)
}
func LogN(a float64) float64 {
	return math.Log(a)
}
func main() {
	var x, y float64
	var ChosenOperation string
	fmt.Print("Enter ur operation:")
	fmt.Scan(&ChosenOperation)
	switch ChosenOperation {
	case "addition":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", Addition(int(x), int(y)))
	case "substraction":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", Substraction(int(x), int(y)))
	case "multiplication":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", Multiplication(int(x), int(y)))
	case "division":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", Division(x, y))
	case "exponentiation":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", Exponentiation(x, y))
	case "rooting":
		fmt.Print("Enter your 2 variables:")
		fmt.Scan(&x, &y)
		fmt.Printf("Result: %v", SquareRoot(x, y))
	case "log10":
		fmt.Print("Enter your variable:")
		fmt.Scan(&x)
		fmt.Printf("Result: %v", Log10(x))
	case "logn":
		fmt.Print("Enter your variable:")
		fmt.Scan(&x)
		fmt.Printf("Result: %v", LogN(x))
	default:
		fmt.Print("You chose a wrong option")
	}
}
