package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Printf("\033[32mEnter the mode(e/d)\033[0m: ")
	scanner.Scan()
	mode := scanner.Text()
	fmt.Printf("\033[33mEnter the data\033[0m: ")
	scanner.Scan()
	data := scanner.Text()

	if mode == "e" {
		encode(data)
	} else if mode == "d" {
		decode(data)
	}
}

func encode(data string) {

	buffer_len := len(data)

	var dump string
	for i := 0; i < buffer_len; i++ {

		if !char_validate(int16(data[i])) {
			fmt.Fprintln(os.Stderr, "InputError: can't take non-ascii characters")
			os.Exit(1)
		}

		binary := fmt.Sprintf("%08b", data[i])
		dump += binary
	}
	dump_size := len(dump)
	for dump_size%6 != 0 {
		dump += "0"
		dump_size++
	}

	i, j := 0, 0
	var base64 string

	for i = 0; i < len(dump)/6; i++ {
		six_bit_bin := dump[j : j+6]
		ascii_value, _ := strconv.ParseUint(six_bit_bin, 2, 64)
		if ascii_value >= 0x0 && ascii_value <= 0x19 {
			base64 += string(rune(ascii_value + 0x41))
		} else if ascii_value >= 0x1a && ascii_value <= 0x33 {
			base64 += string(rune(ascii_value + 0x47))
		} else if ascii_value >= 0x34 && ascii_value <= 0x3d {
			base64 += string(rune(ascii_value - 0x4))
		} else if ascii_value == 0x3e {
			base64 += string(rune(ascii_value - 0x13))
		} else if ascii_value == 0x3f {
			base64 += string(rune(ascii_value - 0x10))
		}
		j += 6
	}
	for i%4 != 0 {
		base64 += "="
		i++
	}
	fmt.Println(base64)
}

func decode(data string) {

	data = strings.ReplaceAll(data, " ", "")
	data = strings.ReplaceAll(data, "=", "")
	datalen := len(data)
	var dump string

	for i := 0; i < datalen; i++ {

		encoded_character := data[i]
		if !base64_validate(int16(encoded_character)) {
			fmt.Fprintln(os.Stderr, "InputError: the string to be decoded is not correctly encoded")
			os.Exit(1)
		}

		var Ox49_val_bin string
		if encoded_character >= 'A' && encoded_character <= 'Z' {
			Ox49_val_bin = fmt.Sprintf("%06b", encoded_character-65)
		} else if encoded_character >= 'a' && encoded_character <= 'z' {
			Ox49_val_bin = fmt.Sprintf("%06b", encoded_character-71)
		} else if encoded_character >= '0' && encoded_character <= '9' {
			Ox49_val_bin = fmt.Sprintf("%06b", encoded_character+4)
		} else if encoded_character == '+' {
			Ox49_val_bin = fmt.Sprintf("%06b", encoded_character+19)
		} else {
			Ox49_val_bin = fmt.Sprintf("%06b", encoded_character+16)
		}
		dump += Ox49_val_bin
	}

	dumplen := len(dump)
	for dumplen%8 != 0 {
		dump += "0"
		dumplen++
	}
	i, j := 0, 0
	var decodeData string
	for i = 0; i < len(dump)/8; i++ {
		byte_bin := dump[j : j+8]
		decimal, _ := strconv.ParseUint(byte_bin, 2, 64)
		decodeData += string(rune(decimal))
		j += 8
	}

	fmt.Println(decodeData)
}
