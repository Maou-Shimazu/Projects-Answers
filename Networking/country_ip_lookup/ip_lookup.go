package main

import (
	"fmt"
  "log"
  "net/http"
  "encoding/json"
  "io/ioutil"
)

type JSON struct {
  Country string `json:"country"`
}

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
  var country JSON
  json.Unmarshal(data, &country)
  fmt.Println(country.Country)
}
