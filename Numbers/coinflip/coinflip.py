import random

count = input("Enter the amount of times you want the coin flipped... ")

choices = ["Heads", "Tails"]
results = []

for i in range(0, int(count)):
    result = random.choice(choices)
    results.append(result)

print(results)
