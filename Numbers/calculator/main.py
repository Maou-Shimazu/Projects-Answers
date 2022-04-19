while True:
  eq = input("pythonCalcRepl: ")
  if eq.upper().isupper() == True:
    print("Invalid equation.")
  else:
    print(eval(eq))
