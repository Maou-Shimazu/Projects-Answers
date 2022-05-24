class Rectangle:
  def __init__(self, width, length):
    self.width = width
    self.length = length

  def Area(self):
    return self.width * self.length

class Circle:
  def __init__(self, radius):
    self.radius = radius

  def Area(self):
    return 3.14 * (self.radius * self.radius)

myRectangle = Rectangle(10, 10)
print(myRectangle.Area())

myCircle = Circle(10)
print(myCircle.Area())
