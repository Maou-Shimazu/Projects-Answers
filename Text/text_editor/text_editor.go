// I might turn this into a full featured text editor at https://github.com/brandondoesdev, keep an eye out

package main

import (
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
	"log"
	"os"
)

func main() {
	myApp := app.New()
	myWindow := myApp.NewWindow("Entry Widget")

	input := widget.NewEntry()
	input.MultiLine = true
	input.SetPlaceHolder("Enter text...")

	dir := widget.NewEntry()
	dir.SetPlaceHolder("Enter the directory you want it to go in...")

	name := widget.NewEntry()
	name.SetPlaceHolder("Enter the desired file name...")

	content := container.NewVBox(input, dir, name, widget.NewButton("Save", func() {
		data := []byte(input.Text)
		os.Chdir(dir.Text)
		_, err := os.Create(name.Text)
		if err != nil {
			log.Fatal(err)
		}
		_err := os.WriteFile(name.Text, data, 0666)
		if _err != nil {
			log.Fatal(err)
		}
	}))

	myWindow.SetContent(content)
	myWindow.ShowAndRun()
}
