isPalindrome :: String -> Bool
isPalindrome s = s == reverse s

main = do
  putStrLn "Enter a word: "
  text <- getLine
  print(show(isPalindrome text))
