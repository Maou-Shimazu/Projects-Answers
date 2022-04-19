package main

import (
  "fmt"
  "regexp"
)

func Verify(str string, regex string) string {
	exp, err := regexp.Compile(regex)
  if err != nil {
    return err.Error()
  }
	r := exp.FindString(str)
	return r
}

func main() {
	fmt.Println("Enter the string...")
  var str string
  fmt.Scan(&str)
  fmt.Println("Enter the regular expression...")
  var regex string
  fmt.Scan(&regex)
  fmt.Println(Verify(str, regex))
}
