loc = input("Enter the number you want to calculate up to... ")
sequence = [0, 1]
for i in range(1, int(loc) + 1):
  sum = sequence[i] + sequence[i-1]
  sequence.append(sum)
print(sequence)
