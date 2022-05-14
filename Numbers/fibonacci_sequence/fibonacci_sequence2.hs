fibs :: Num a => [a]
fibs = 0 : 1 : zipWith (+) fibs (tail fibs)

main :: IO ()
main = do
  putStr "Number of fibonacci numbers to calculate: "
  n <- readLn
  print . take n $ fibs
