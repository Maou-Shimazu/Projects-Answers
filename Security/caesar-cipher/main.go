package main

import (
    "fmt"
    "strings"
)

func main() {
    alphabet := [] string {"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
  
    fmt.Println("Please enter the mode you want...")
    var mode string
    fmt.Scan(&mode)
    mode = strings.ToLower(mode)
  
    if mode == "encrypt" {
      fmt.Println("Please enter the message you want encrypted...")
      var message string
      fmt.Scan(&message)
      
      fmt.Println("Please enter the shift number...")
      var shift int
      fmt.Scan(&shift)
      
      var newmsg string

      for i := range message {
        
        letter := string(message[i])
        var index int
        
        for ai := range alphabet {
          if alphabet[ai] == letter {
            index = ai
          }
        }
        
        if (index + shift) > len(alphabet) {
          for (index + shift) > len(alphabet) {
            shift -= len(alphabet)
          }
        }
        newmsg += alphabet[index + shift]
      }
      fmt.Println(newmsg)
    } else if mode == "decrypt" {
      fmt.Println("Please enter the message you want decrypted...")
      var message string
      fmt.Scan(&message)
      
      fmt.Println("Please enter the shift number...")
      var shift int
      fmt.Scan(&shift)
      
      var newmsg string

      for i := range message {
        
        letter := string(message[i])
        var index int
        
        for ai := range alphabet {
          if alphabet[ai] == letter {
            index = ai
          }
        }
        
        if (index + shift) > len(alphabet) {
          for (index + shift) > len(alphabet) {
            shift -= len(alphabet)
          }
        }
        
        newmsg += alphabet[index - shift]
      }
      fmt.Println(newmsg)
    }
}
