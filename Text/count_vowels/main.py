word = input('Enter a word: ')

vowels = 0 

for letter in word:
    if letter in 'aeiou':
        vowels += 1
        
print(f'There are {vowels} vowels in {word}')
