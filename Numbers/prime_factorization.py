import sys
import math
sys.setrecursionlimit(1000)

def leastPrimeFactor(x: int) -> int:
    while x % 2 == 0:
        print(f"LPF of {x}: {2}")
        x = x // 2
    for i in range(3, int(math.sqrt(x))+1, 2):
        while x % i== 0:
            print(f"LPF of {x}: {i}"),
            x = x // i
    if x > 2:
        print(x)

print("Input a number: ", end="")
try:
	num: int = int(input())
	leastPrimeFactor(num)
    
except ValueError:
	print("Type not valid. Please input a number.")
