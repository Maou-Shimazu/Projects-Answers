package main

import (
	"fmt"
  "time"
  "os"
  "log"
  "github.com/faiface/beep"
  "github.com/faiface/beep/speaker"
  "github.com/faiface/beep/mp3"
)

func main() {
	fmt.Println("Please enter the time in minutes you want the alarm to last...")

  var wait float64
  fmt.Scan(&wait)

  time.Sleep(time.Duration(wait) * time.Minute)

  f, err := os.Open("EXAMPLE.mp3")
	if err != nil {
		log.Fatal(err)
	}

  streamer, format, err := mp3.Decode(f)
	if err != nil {
		log.Fatal(err)
	}
  defer streamer.Close()

	speaker.Init(format.SampleRate, format.SampleRate.N(time.Second/10))

  speaker.Play(streamer)
	select {}
}
