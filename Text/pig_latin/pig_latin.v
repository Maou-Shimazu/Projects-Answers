import readline { read_line }

fn main() {
	for i := 0; true; i++ {
		mut answer := read_line('Enter your word (q to exit): ')?.trim_space()
		if answer == 'q' {
			break
		}
		pl := answer.replace(answer[0].ascii_str(), "") + answer[0].ascii_str() + "ay"
		println(pl)
		println("")
	}
	
}