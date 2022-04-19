package main

import (
	"fmt"
  "math/rand"
  "time"
)

func main() {
  choices := []string{"Heads", "Tails"}
  time := time.Now()
  unix := time.Unix()
  rand.Seed(unix)
  index := rand.Intn(len(choices))
  fmt.Println(choices[index])
}
