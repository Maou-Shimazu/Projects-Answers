import os {input}
fn main() {
    mut s := input("Word to check for palindrome: ")
	if s == s.reverse() {
		println("That is a palindrome!")
	}
	else { println("That is not a palindrome!") }
    
}