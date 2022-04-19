word = input("Enter a word: ") 

rev = word[::-1]

if(word == rev):
    print("The word is a palindrome")
else: 
    print("The word is not a palindrome")
