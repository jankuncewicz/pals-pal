package main

import (
	"fmt"
	"math"
)

func zero_newton(n, z int) {
	// Initial guess (where does it come from?)
	// TODO: substitute for z
	x0 := 1 + math.Sqrt(2) + float64(z-1)*math.Pi + float64(n) + math.Pow(float64(n), 0.4)
	fmt.Println(x0)
}

func main() {
	fmt.Println(math.Jn(-3, 2))
}
