import string

alphabet_str = string.ascii_lowercase
alphabet = list(alphabet_str)

type = input("Would you like to encrypt or decrypt a message?")

if type == "encrypt":
  msg = input("Enter the message you want encrypted...")
  shift = int(input("Enter the shift number..."))
  new_msg = ""
  for letter in msg:
    if alphabet.index(letter) + shift > len(alphabet):
      while alphabet.index(letter) + shift > len(alphabet):
        shift -= len(alphabet)
    new_msg += alphabet[alphabet.index(letter) + shift]
  print(new_msg)
if type == "decrypt":
  msg = input("Enter the message you want decrypted...")
  shift = int(input("Enter the shift number..."))
  new_msg = ""
  for letter in msg:
    if alphabet.index(letter) + shift > len(alphabet):
      while alphabet.index(letter) + shift > len(alphabet):
        shift -= len(alphabet)
    new_msg += alphabet[alphabet.index(letter) - shift]
  print(new_msg)
