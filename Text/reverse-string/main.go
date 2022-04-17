package main

import "fmt"

func main() {
	fmt.Println("Enter the string you want reversed...")
    var str string
    fmt.Scan(&str)
    for i := len(str); i > 0; i-- {
      fmt.Println(string(str[i - 1]))
    }
}
