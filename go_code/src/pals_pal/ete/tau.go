package ete

import (
	"math"
)

func g(m int) float64 {
	if m == 0 {
		return 1
	}
	return 2
}

func Tau(n, m int, R, delta, T float64, zeros, ints [][]float64) float64 {
	sum1 := 0.0
	sum2 := 0.0
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			// znm -- n-th zero of the m-th kind
			exp := math.Exp(Enm(zeros[i][j], R, delta, T))
			sum1 += g(i) * exp
			sum2 += g(i) * exp * Lambda_nm(P_nm(j, i, R, delta, zeros[i][j], ints[i][j]))
		}
	}
	return sum1 / sum2
}
