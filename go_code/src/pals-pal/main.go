package main

import (
	"fmt"
	"pals_pal/bessel"
	"pals_pal/ete"
)

func main() {
	n := 100
	m := 50
	zeros := bessel.Zeros(n, m)
	ints := bessel.IntegralList(n, m, zeros)
	fmt.Printf("R\ttau1\ttau2\n")
	for i := .1; i < .99; i += .1 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n, m, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 200, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 100, zeros, ints))
	}
	for i := 1.0; i < 10; i += .2 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n, m, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 200, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 100, zeros, ints))
	}
	for i := 10.0; i <= 100; i += 5 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n, m, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 200, zeros, ints),
			ete.Tau(n, m, i/2, 0.193, 100, zeros, ints))
	}
}
