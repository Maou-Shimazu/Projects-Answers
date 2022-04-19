getNum :: (Read a, Num a, Show a) => IO a
getNum = readLn

fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)

main = do
  putStrLn "Enter a cap: "
  cap <- getNum
  print(map fib [1..cap])
