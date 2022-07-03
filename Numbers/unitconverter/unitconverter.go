package main

import "fmt"

type unitConverter interface {
	convert(x float64) float64
}

type convertFunc func(x float64) (float64, float64)
type convertFunc2 func(x float64) (float64, float64, float64)

func (c convertFunc) convert(x float64) (float64, float64) {
	return c(x)
}
func (c convertFunc2) convert(x float64) (float64, float64, float64) {
	return c(x)
}
func ConvertFahrenheit(x float64) (float64, float64) {
	Celsius := (x - 32) / 1.8
	Kelvin := (x-32)/1.8 + 273.15
	return Celsius, Kelvin
}
func ConvertCelsius(x float64) (float64, float64) {
	Fahrenheit := (x * 1.8) + 32
	Kelvin := x + 273.15
	return Fahrenheit, Kelvin
}
func ConvertKelvin(x float64) (float64, float64) {
	Fahrenheit := (x-273.15)*1.8 + 32
	Celsius := x - 273.15
	return Fahrenheit, Celsius
}
func ConvertDollar(x float64) (float64, float64, float64) {
	Euro := x * 0.94935403
	Hryvnia := x * 29.543471
	Ruble := x * 53.796937
	return Euro, Hryvnia, Ruble
}
func ConvertEuro(x float64) (float64, float64, float64) {
	Dollar := x * 1.0533478
	Hryvnia := x * 31.099677
	Ruble := x * 56.634715
	return Dollar, Hryvnia, Ruble
}
func ConvertHryvnia(x float64) (float64, float64, float64) {
	Dollar := x * 0.033870198
	Euro := x * 0.032165364
	Ruble := x * 1.8236556
	return Dollar, Euro, Ruble
}
func ConvertRuble(x float64) (float64, float64, float64) {
	Dollar := x * 0.01860151
	Euro := x * 0.017654294
	Hryvnia := x * 0.54882212
	return Dollar, Euro, Hryvnia
}
func ConvertKilogram(x float64) (float64, float64, float64) {
	Gram := x * 1000
	Pound := x * 2.2046226218
	Ounce := x * 35.27396195
	return Gram, Pound, Ounce
}
func ConvertGram(x float64) (float64, float64, float64) {
	Kilogram := x * 0.001
	Pound := x * 0.0022046226
	Ounce := x * 0.0352739619
	return Kilogram, Pound, Ounce
}
func ConvertPound(x float64) (float64, float64, float64) {
	Gram := x * 453.59237
	Kilogram := x * 0.45359237
	Ounce := x * 16
	return Gram, Kilogram, Ounce
}
func ConvertOunce(x float64) (float64, float64, float64) {
	Gram := x * 28.349523125
	Kilogram := x * 0.0283495231
	Pound := x * 0.0625
	return Gram, Kilogram, Pound
}

func main() {
	var Unit string
	var number float64
	fahrenheit := convertFunc(ConvertFahrenheit)
	celsius := convertFunc(ConvertCelsius)
	kelvin := convertFunc(ConvertKelvin)
	dollar := convertFunc2(ConvertDollar)
	euro := convertFunc2(ConvertEuro)
	hryvnia := convertFunc2(ConvertHryvnia)
	ruble := convertFunc2(ConvertRuble)
	kilogram := convertFunc2(ConvertKilogram)
	gram := convertFunc2(ConvertGram)
	pound := convertFunc2(ConvertPound)
	ounce := convertFunc2(ConvertOunce)
	fmt.Print("Enter the unit to convert:")
	fmt.Scan(&Unit)
	fmt.Print("Enter the number of units:")
	fmt.Scan(&number)
	switch Unit {
	case "fahrenheit":
		x, y := fahrenheit.convert(number)
		fmt.Printf("In %v fahrenheits: %v celsius and %v kelvins", number, x, y)
	case "celsius":
		x, y := celsius.convert(number)
		fmt.Printf("In %v celsius: %v fahrenheits and %v kelvins", number, x, y)
	case "kelvin":
		x, y := kelvin.convert(number)
		fmt.Printf("In %v kelvins: %v fahrenheits and %v celsius", number, x, y)
	case "dollar":
		x, y, z := dollar.convert(number)
		fmt.Printf("In %v dollars: %v euros, %v hryvnias and %v rubles", number, x, y, z)
	case "euro":
		x, y, z := euro.convert(number)
		fmt.Printf("In %v euros: %v dollars, %v hryvnias and %v rubles", number, x, y, z)
	case "hryvnia":
		x, y, z := hryvnia.convert(number)
		fmt.Printf("In %v hryvnias: %v dollars, %v euros and %v rubles", number, x, y, z)
	case "ruble":
		x, y, z := ruble.convert(number)
		fmt.Printf("In %v rubles: %v dollars, %v euros and %v hryvnias", number, x, y, z)
	case "kilogram":
		x, y, z := kilogram.convert(number)
		fmt.Printf("In %v kilograms: %v grams, %v pounds and %v ounces", number, x, y, z)
	case "gram":
		x, y, z := gram.convert(number)
		fmt.Printf("In %v grams: %v kilograms, %v pounds and %v ounces", number, x, y, z)
	case "pound":
		x, y, z := pound.convert(number)
		fmt.Printf("In %v pounds: %v grams, %v kilograms and %v ounces", number, x, y, z)
	case "ounce":
		x, y, z := ounce.convert(number)
		fmt.Printf("In %v ounces: %v grams, %v kilograms and %v pounds", number, x, y, z)
	}
}
