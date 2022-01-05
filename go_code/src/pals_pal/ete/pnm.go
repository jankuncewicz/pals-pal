package ete

import "pals_pal/bessel"

func P_nm(n, m int, R, delta, zero, ints float64) float64 {
	return bessel.Integrate_gauss(zero*(R/(R+delta)), zero, m) / ints
}
