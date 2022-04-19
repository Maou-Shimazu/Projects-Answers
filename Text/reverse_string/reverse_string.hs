main = do
  putStrLn "Enter the string you want reversed..."
  text <- getLine
  putStrLn(reverse text)
