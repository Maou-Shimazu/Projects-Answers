package main

import (
	"fmt"
  "log"
  "net/http"
  "io/ioutil"
)

func main() {
  fmt.Println("Please enter the IP address...")
  var ip string
  fmt.Scan(&ip)
	response, err := http.Get("http://ip-api.com/json/" + ip)
  if err != nil {
     log.Fatalln(err)
  }
  data, err := ioutil.ReadAll(response.Body)
  if err != nil {
    log.Fatal(err)
  }
  fmt.Println(string(data[:]))
}
