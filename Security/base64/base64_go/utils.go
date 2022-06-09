package main

func char_validate(char int16) bool {
	if char < 0 || char > 126 {
		return false
	}
	return true
}

func base64_validate(b64ec int16) bool {

	if b64ec < 43 || b64ec > 126 || (b64ec >= 44 && b64ec < 47) {
		return false
	} else if (b64ec > 57 && b64ec < 65) || (b64ec > 90 && b64ec < 97) {
		return false
	} else if b64ec > 122 && b64ec < 126 {
		return false
	}
	return true
}
