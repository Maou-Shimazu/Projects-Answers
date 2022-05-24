package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func generateKey(key, cred string) string {

	var i int

	cred_len := len(cred)
	key_len := len(key)
	var nkey []rune
	for i = 0; i < key_len; i++ {
		if unicode.IsLetter(rune(key[i])) {
			nkey = append(nkey, rune(key[i]))
		}
	}

	if string(nkey) == "" {
		return ""
	}

	genKey := make([]rune, cred_len)
	key_len = len(nkey)
	j := 0
	for i = 0; i < cred_len; i++ {

		if j >= key_len {
			j = 0
		}
		tmp := rune(cred[i])
		if !unicode.IsLetter(tmp) {
			genKey[i] = tmp
		} else {
			genKey[i] = nkey[j]
			j++
		}
	}
	return string(genKey)
}

func eval(cred, key, mode string) {

	var i int
	cred_len := len(cred)
	res := make([]rune, cred_len)

	if mode == "encrypt" {
		for i = 0; i < cred_len; i++ {

			runed := rune(cred[i])
			// checks if cred element at index i is between A or Z
			if unicode.ToUpper(runed) >= 65 && unicode.ToUpper(runed) <= 90 {
				//res is appended with encrypted char
				res[i] = unicode.ToUpper(runed) - (65 - unicode.ToUpper(rune(key[i])))

				// checks if the encrypted char is greater than Z
				if res[i] > 90 {
					//encrypted character is subtracted with 26, so it's between A and Z
					res[i] -= 26
				}
			} else { // if element is a non-alphabetic character, res is appended with the element
				res[i] = runed
			}

			// matching case in both data and encrypted data
			if unicode.IsLower(runed) {
				res[i] += 32
			}
		}
	} else if mode == "decrypt" {
		for i = 0; i < cred_len; i++ {

			runed := rune(cred[i])
			// checks if cred element at index i is between A or Z
			if unicode.ToUpper(runed) >= 65 && unicode.ToUpper(runed) <= 90 {
				//res is appended with encrypted char
				res[i] = unicode.ToUpper(runed) - (unicode.ToUpper(rune(key[i])) - 65)

				// checks if the encrypted char is greater than Z
				if res[i] < 65 {
					//encrypted character is subtracted with 26, so it's between A and Z
					res[i] += 26
				}
			} else { // if element is a non-alphabetic character, res is appended with the element
				res[i] = runed
			}

			// matching case in both data and encrypted data
			if unicode.IsLower(runed) {
				res[i] += 32
			}
		}
	} else {
		fmt.Fprintf(os.Stderr, "\033[31mModeError: %s is not a valid mode\033[0m\n", mode)
		os.Exit(1)
	}

	fmt.Println(string(res))
}

func main() {

	scanner := bufio.NewScanner(os.Stdin)
	fmt.Printf("\033[32mEnter the mode you want to perform\033[0m: ")
	scanner.Scan()
	mode := scanner.Text()
	fmt.Printf("\033[33mEnter the data string you want to %s\033[0m: ", mode)
	scanner.Scan()
	data := scanner.Text()
	fmt.Printf("\033[36mEnter the key you want your data to %s with\033[0m: ", mode)
	scanner.Scan()
	key := scanner.Text()
	genKey := generateKey(key, data)
	eval(data, genKey, mode)
}
