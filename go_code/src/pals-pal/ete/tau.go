package ete

import "math"

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
			exp := math.Exp(Enm(zeros[i][j], R, delta, T))
			sum1 += g(m) * exp
			sum2 += g(m) * Lambda_nm(P_nm(i, j, R, delta, zeros[i][j], ints[i][j])) * exp
		}
	}
	return sum1 / sum2
}
