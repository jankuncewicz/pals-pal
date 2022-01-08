package bessel

import "math"

func Integrate(start, end float64, m int) float64 {
	const acc = 50
	delta := (end - start) / acc
	ans := 0.0
	for i := 0; i < acc; i++ {
		mid := (2*start + float64(2*i+1)*delta) / 2
		ans += math.Pow(math.Jn(m, mid), 2) * delta
	}
	return ans
}

func Integrate_gauss(start, end float64, m int) float64 {
	x := []float64{.14887, .43340, .67941, .86506, .97391}
	w := []float64{.29552, .26927, .21909, .14945, .06667}
	ans := 0.0
	a := (start + end) / 2
	b := (end - start) / 2
	for i := 0; i < 5; i++ {
		ans += w[i] * math.Pow(math.Jn(m, b*x[i]+a), 2) * (b*x[i] + a)
		ans += w[i] * math.Pow(math.Jn(m, -b*x[i]+a), 2) * (-b*x[i] + a)
	}
	return b * ans
}

func IntegralList(v, m int, zeros, ints [][]float64) [][]float64 {
	// v -- max degree - 1
	// m -- max zero no.
	var beg int
	len_ints_row := len(ints)
	var len_ints_col int
	if len_ints_col == 0 {
		len_ints_col = 0
	} else {
		len_ints_col = len(ints[0])
	}
	ans := make([][]float64, v)
	for i := 0; i < v; i++ {
		hold := make([]float64, m)
		if i < len_ints_row {
			beg = len_ints_col
		} else {
			beg = 0
		}
		for j := 0; j < beg; j++ {
			hold[j] = ints[i][j]
		}
		if beg == 0 {
			hold[0] = Integrate_gauss(0, zeros[i][0], i)
			beg++
		}
		for j := beg; j < m; j++ {
			hold[j] = hold[j-1] + Integrate_gauss(zeros[i][j-1], zeros[i][j], i)
		}
		ans[i] = hold
	}
	return ans
}
