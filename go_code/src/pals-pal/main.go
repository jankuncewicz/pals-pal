package main

import (
	"fmt"
	"pals_pal/bessel"
	"pals_pal/ete"
)

func main() {
	// TODO: guess R having tau
	n1 := 100
	n2 := 50
	n3 := 10
	zeros := bessel.Zeros(n1, n1)
	ints := bessel.IntegralList(n1, n1, zeros)
	for i := .1; i < .99; i += .1 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n1, n1, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n2, n2, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n3, n3, i/2, 0.193, 300, zeros, ints))
	}
	for i := 1.0; i < 9.9; i += .2 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n1, n1, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n2, n2, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n3, n3, i/2, 0.193, 300, zeros, ints))
	}
	for i := 10.0; i <= 100; i += 5 {
		fmt.Printf("%f\t%f\t%f\t%f\n", i,
			ete.Tau(n1, n1, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n2, n2, i/2, 0.193, 300, zeros, ints),
			ete.Tau(n3, n3, i/2, 0.193, 300, zeros, ints))
	}
}
