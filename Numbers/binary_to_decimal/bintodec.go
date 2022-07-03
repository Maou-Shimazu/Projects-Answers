package main

import (
	"fmt"
	"math"
)

func main() {
	var numbersystem string
	var number, i int
	var ConvertedNumber, count = 0, 0
	fmt.Print("Is your number binary or decimal:")
	fmt.Scanf("%s", &numbersystem)
	if numbersystem == "binary" {
		fmt.Print("Enter your binary number:")
		fmt.Scanf("%d", &number)
		for number != 0 {
			ConvertedNumber += number % 10 * int(math.Pow(float64(2), float64(i)))
			number /= 10
			i++
		}
		fmt.Printf("Your decimal converted from binary %d", ConvertedNumber)
	} else if numbersystem == "decimal" {
		fmt.Print("Enter your decimal number:")
		fmt.Scanf("%d", &number)
		for number != 0 {
			if number%2 == 0 {
				ConvertedNumber += int(1 * math.Pow(10, float64(count)))
				ConvertedNumber -= int(1 * math.Pow(10, float64(count)))
				count++
				number /= 2
			} else if number%2 == 1 {
				ConvertedNumber += int(1 * math.Pow(10, float64(count)))
				count++
				number /= 2
			}
		}
		fmt.Printf("Your binary converted from decimal:%d", ConvertedNumber)
	} else {
		fmt.Print("Not right option")
	}
}
