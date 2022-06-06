card = input("Enter the card number: ")
list = card.split()
sum = 0

try:
  for num in list:
    num = int(num)
    if num * 2 > 9:
      sum += num * 2 - 9
    else:
      sum += num * 2

  if num % 10 == 0:
    print("That is a valid card number!")
  else:
    print("Invalid card number!")
except ValueError:
  print("Please enter a card number")
  
