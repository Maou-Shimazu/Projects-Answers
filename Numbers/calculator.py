print("Easy Python Calculator")
print('''
-- Options --
> Addition / addition / +
> Subtraction / subtraction / -
> Multiplication / multiplication / *
> Division / division / /
''')
operator = input("Enter the desired operator... ")
fnum = float(input("Enter the first number... "))
snum = float(input("Enter the second number... "))
if operator == "Addition" or "addition" or "+":
    result = fnum + snum
    print(f"Your result: {result}")
elif operator == "Subtraction" or "subtraction" or "-":
    result = fnum - snum
    print(f"Your result: {result}")
elif operator == "Multiplication" or "multiplication" or "*":
    result = fnum * snum
    print(f"Your result: {result}")
elif operator == "Division" or "division" or "/":
    result = fnum / snum
    print(f"Your result: {result}")
else:
    print("Invalid input")
