isPalindrome :: String -> Bool
isPalindrome s = s == reverse s

-- pointfree style
-- isPalindrome :: String -> Bool
-- isPalindrome = (==) <*> reverse

main = do
  putStrLn "Enter a word: "
  text <- getLine
  print(show(isPalindrome text))
