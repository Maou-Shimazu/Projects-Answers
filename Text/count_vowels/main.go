package main

import (
	"fmt"
)

func main() {
	vowels := []string{"A", "E", "I", "O", "U", "a", "e", "i", "o", "u"}
  var counter int
  fmt.Println("Enter your word: ")
  var word string
  fmt.Scan(&word)
  for i := range word {
    letter := string(word[i])
    for i2 := range vowels {
      vowel := string(vowels[i2])
      if letter == vowel {
        counter += 1
      }
    }
  }
  fmt.Println(counter)
}
