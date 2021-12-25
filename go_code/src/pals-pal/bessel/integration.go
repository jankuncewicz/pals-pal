package bessel

import "math"

func Integrate(start, end float64, m int) float64 {
	// TODO: better integration algorithm
	const acc = 50
	delta := (end - start) / acc
	ans := 0.0
	for i := 0; i < acc; i++ {
		//mid := delta * (2*float64(i) + 1) / 2
		mid := (2*start + float64(2*i+1)*delta) / 2
		ans += math.Pow(math.Jn(m, mid), 2) * delta
	}
	return ans
}

func IntegralList(v, m int, zeros [][]float64) [][]float64 {
	// v -- max degree - 1
	// m -- max zero no.
	ints := make([][]float64, v)
	hold := make([]float64, m)
	for i := 0; i < v; i++ {
		hold[0] = Integrate(0, zeros[i][0], i)
		for j := 1; j < m; j++ {
			hold[j] = hold[j-1] + Integrate(zeros[i][j-1], zeros[i][j], i)
		}
		ints[i] = hold
		hold = make([]float64, m)
	}
	return ints
}
