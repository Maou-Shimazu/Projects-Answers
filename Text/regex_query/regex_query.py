import re

def Verify(str, regex):
  try:
    result = re.findall(regex, str)
  except Exception as e:
    return e
  return result

str = input("Enter the string... ")
regex = input("Enter the regular expression... ")
print(Verify(str, regex))
