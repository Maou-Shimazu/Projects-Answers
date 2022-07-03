package main

import (
	"fmt"
	"os"
)

func main() {
	var UserNumber, count int
	var arr = [4]int{2, 3, 5, 7}
	fmt.Print("Enter a number to factorize:")
	fmt.Scanf("%d", &UserNumber)
	fmt.Fprintf(os.Stdout, "Prime factorization of %d:\n", UserNumber)
	for count < 4 {
		count = 0
		for i := 0; i < len(arr); i++ {
			if UserNumber%arr[i] == 0 {
				UserNumber /= arr[i]
				fmt.Fprintf(os.Stdout, "%d ", arr[i])
			} else if UserNumber%arr[i] != 0 {
				count += 1
			}
		}
	}
	if UserNumber != 1 {
		fmt.Fprint(os.Stdout, "\nNah, it already is a prime number")
	}

	var Continue string
	fmt.Print("\nHere are some prime numbers:\n")
	for i := 2; i < 10000; i++ {
		if i < 10 {
			if i/2 == 1 && i%2 == 0 || i/3 == 1 && i%3 == 0 || i/5 == 1 && i%5 == 0 || i/7 == 1 && i%7 == 0 {
				fmt.Printf("%d", i)
				fmt.Print("\nDo you want to continue listing?\n")
				fmt.Scan(&Continue)
				if Continue != "yes" {
					break
				}
			}
		} else if i%2 != 0 && i%3 != 0 && i%5 != 0 && i%7 != 0 {
			fmt.Printf("%d ", i)
			fmt.Print("\nDo you want to continue listing?\n")
			fmt.Scan(&Continue)
			if Continue != "yes" {
				break
			}
		}
	}
}
