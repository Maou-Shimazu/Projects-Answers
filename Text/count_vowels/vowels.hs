isVowel :: Char -> Bool
isVowel x = elem x "aeiou"

vowels :: String -> Int
vowels s = length $ filter isVowel s

main = do
  putStrLn "Enter a word: "
  text <- getLine
  print(vowels text)
