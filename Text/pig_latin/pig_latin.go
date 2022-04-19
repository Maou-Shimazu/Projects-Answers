package main

import (
	"fmt"
)

func main() {
	vowels := []string{"a", "e", "i", "o", "u", "A", "E", "I", "O", "U"}
  var break_index int
  fmt.Println("Enter your word...")
  var word string
  fmt.Scan(&word)
  wrapper:
  for i := range word {
    letter := string(word[i])
    for i2 := i; i2 < len(word); i2++ {
      for vowel := range vowels {
        if vowels[vowel] == letter {
          break_index = i
          break wrapper
        }
      }
    }
  }
  fmt.Println(word[break_index:] + "-" + word[:break_index] + "ay")
}
