import math 

binary = input('Enter a binary number: ')

decimal = 0 
for i in range(len(binary)):
    decimal += int(binary[i]) * pow(2, len(binary) - i - 1)

print(decimal)
