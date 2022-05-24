package main

import (
	"fmt"
)

type Rectangle struct {
  width float32;
  height float32;
}

func (r Rectangle) Area() float32 {
  area := r.width * r.height
  return area
}

type Circle struct {
  radius float32;
}

func (c Circle) Area() float32 {
  area := 3.14 * (c.radius * c.radius)
  return area
}

func main() {
	myRectangle := Rectangle { width: 10, height: 10 }
  fmt.Println(myRectangle.Area())
  myCircle := Circle { radius: 10 }
  fmt.Println(myCircle.Area())
}
