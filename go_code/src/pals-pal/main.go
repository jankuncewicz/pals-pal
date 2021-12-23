package main

import (
	"fmt"
	"pals_pal/bessel"
	"pals_pal/ete"
)

func main() {
	m := 100
	n := 50
	zeros := bessel.Zeros(m, n)
	ints := make([][]float64, m)
	hold := make([]float64, n)
	for i := 0; i < m; i++ {
		hold[0] = bessel.Integrate(0, zeros[i][0], i)
		for j := 1; j < n; j++ {
			hold[j] = hold[j-1] + bessel.Integrate(zeros[i][j-1], zeros[i][j], i)
		}
		ints[i] = hold
		hold = make([]float64, n)
	}
	for i := 1.0; i < 10000; i += 50 {
		fmt.Println(i/100, ete.Tau(m, n, i/2, 0.193, 300, zeros, ints))
	}
}
