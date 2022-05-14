data Rectangle = Rectangle Float Float deriving (Show)
newtype Circle = Circle Float deriving (Show)

class Shape a where
  area      :: a -> Float
  perimeter :: a -> Float

-- 2 implementations, one for Rectangle and one for Circle
instance Shape Rectangle where
  area      (Rectangle w h) = w * h
  perimeter (Rectangle w h) = 2 * (w + h)

instance Shape Circle where
  area      (Circle r) = pi * r * r
  perimeter (Circle r) = 2 * pi * r

-- now we can constrain functions to any type with a Shape instance, for example:
shapeInfo :: Shape a => a -> String
shapeInfo s = "Area: " <> show (area s) <> ", Perimeter: " <> show (perimeter s)
