package main

import (
	"fmt"
)

func main() {
	fmt.Println("Please enter the number in which you want prime numbers calculated up to...")
  
  var upto int
  fmt.Scan(&upto)

  num_list := []int{}
  not_prime := []int{}

  for i := 0; i < upto; i++ {
    num_list = append(num_list, i)
  }

  for i := range num_list {
    if (num_list[i] % 2 == 0) && (i >= 2*2) {
      not_prime = append(not_prime, i)
    } else if (i % 3 == 0) && (i >= 3*3) {
      not_prime = append(not_prime, i)
    } else if (i % 5 == 0) && (i >= 5*5) {
      not_prime = append(not_prime, i)
    }
  }

  for i := range not_prime {
    var index int
    for i2 := range num_list {
      if not_prime[i] == num_list[i2] {
        index = i2
      }
    }
    num_list = append(num_list[:index], num_list[index+1:]...)
  }

  fmt.Println("Prime numbers found up to %v", upto)
  fmt.Println(num_list)
}
