package main

import (
	"fmt"
	"os"
)

func main() {
	var price, payment float32
	var change, QuartersForChange, DimesForChange, NickelsForChange, PenniesForChange int
	var enought bool
	quarters, dimes, nickels := 25, 10, 5
	fmt.Print("Enter the price of a purchase in dollars:")
	fmt.Scan(&price)
	fmt.Print("Enter the amount you pay in dollars:")
	fmt.Scan(&payment)
	change = int(payment*100 - price*100)
	if change < 0 {
		change *= -1
		fmt.Printf("You have to pay %.2f more dollars\n", float32(change)/100)
		enought = false
	} else if change == 0 {
		fmt.Printf("You have paid the exact price")
		os.Exit(0)
	}
	if change/quarters >= 1 {
		QuartersForChange = change / quarters
		change -= QuartersForChange * quarters
	} else {
		QuartersForChange = 0
	}
	if change/dimes >= 1 {
		DimesForChange = change / dimes
		change -= DimesForChange * dimes
	} else {
		DimesForChange = 0
	}
	if change/nickels >= 1 {
		NickelsForChange = change / nickels
		change -= NickelsForChange * nickels
	} else {
		NickelsForChange = 0
	}
	if change < 5 {
		PenniesForChange = change
	} else {
		PenniesForChange = 0
	}
	if !enought {
		fmt.Printf("Quarters to pay = %d,\nDimes to pay = %d,\nNickels to pay = %d,\nPennies to pay = %d", QuartersForChange, DimesForChange, NickelsForChange, PenniesForChange)
	} else {
		fmt.Printf("Quarters needed for change = %d,\nDimes needed = %d,\nNickels needed = %d,\nPennies needed = %d", QuartersForChange, DimesForChange, NickelsForChange, PenniesForChange)
	}
}
