package main

import (
	"fmt"
)

func main() {
  nums := []int{11}
  
	for {
    i := len(nums) - 1
    num := nums[i]
    if num == 1 {
      break
    }
    odd := num % 2 == 1
    if odd {
      nums = append(nums, 3 * num + 1)
    } else {
      nums = append(nums, num / 2)
    }
  }

  fmt.Println(nums)
}
