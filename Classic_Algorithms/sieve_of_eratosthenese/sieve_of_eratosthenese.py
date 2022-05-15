print("Welcome to the prime number generator, using the Sieve of Eratosthenese algorithm")

num = int(input("Enter the number you want prime numbers calculated up to..."))

num_list = []
not_prime = []

for i in range(1, num + 1):
  num_list.append(i)

for number in num_list:
  if (number % 2 == 0) & (number >= 2*2):
    not_prime.append(number)
  elif (number % 3 == 0) & (number >= 3*3):
    not_prime.append(number)
  elif (number % 5 == 0) & (number >= 5*5):
    not_prime.append(number)

for number in not_prime:
  num_list.remove(number)
  
print(f"Prime numbers: {num_list}")
print(f"Number of prime numbers found: {len(num_list)}")
