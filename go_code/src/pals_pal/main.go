package main

import (
	"fmt"
	"pals_pal/bessel"
	"pals_pal/ete"
)

func main() {
	n := 50
	zeros := bessel.Zeros(n, n)
	ints := bessel.IntegralList(n, n, zeros)
	fmt.Println(ete.ApproxR(21.1, n, n, 0.193, 300, zeros, ints))
}
