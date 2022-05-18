struct Rectangle {
  width: f64,
  length: f64,
}

impl Rectangle {
  fn Area(&self) -> f64 {
    self.width * self.length
  }
}

struct Circle {
  radius: f64
}

impl Circle {
  fn Area(&self) -> f64 {
    3.14 * (self.radius * self.radius)
  }
}

fn main() {
  let my_rectangle = Rectangle {
    width: 10.0,
    length: 10.0,
  };
  println!("{}", my_rectangle.Area());
  let my_circle = Circle {
    radius: 10.0,
  };
  println!("{}", my_circle.Area());
}
