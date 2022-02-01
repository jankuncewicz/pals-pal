package ete

import (
	"math"
)

func zero_newton(x0, tau float64, n, m int, delta, T float64, zeros, ints [][]float64) float64 {
	const acc = 0.001
	err := 1.0
	const d = 0.001
	x := x0
	for i := 0; i < 1000 && math.Abs(err) > acc; i++ {
		f1 := Tau(n, m, x0, delta, T, zeros, ints) - tau
		f2 := Tau(n, m, x0+d*x0, delta, T, zeros, ints) - tau
		deriv := (f2 - f1) / (d * x0)
		x = x0 - f1/deriv
		x = math.Mod(math.Abs(x), 140)
		err = x0 - x
		x0 = x
	}
	return x
}

func ApproxR(tau float64, n, m int, delta, T float64, zeros, ints [][]float64) float64 {
	// TODO: better guess for small T and large tau
	r_guess := 0.25 * math.Pow((39480499*tau)/(1248483*math.Abs(140-tau)), 2.0/3.0)
	return zero_newton(r_guess, tau, n, m, delta, T, zeros, ints)
}
