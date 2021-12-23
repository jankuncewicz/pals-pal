package bessel

import "math"

func Integrate(start, end float64, m int) float64 {
	const acc = 10
	delta := (end - start) / acc
	ans := 0.0
	for i := 0; i < acc; i++ {
		mid := delta * (2*float64(i) + 1) / 2
		ans += math.Pow(math.Jn(m, mid), 2) * delta
	}
	return ans
}
