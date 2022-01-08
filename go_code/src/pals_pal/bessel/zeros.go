package bessel

import (
	"math"
)

func zero_newton(n int, x0 float64) float64 {
	const acc = 0.001
	err := 1.0
	var x float64
	for math.Abs(err) > acc {
		b_hold := math.Jn(n, x0)
		x = x0 - b_hold/(math.Jn(n-1, x0)-(float64(n)/x0)*b_hold)
		err = x0 - x
		x0 = x
		//fmt.Println(x)
	}
	return x
}

func bessel_guess(v, m int) float64 {
	mu := float64(4 * v * v)
	a := (float64(m) + (1.0/2)*float64(v) - (1.0 / 4)) * math.Pi
	x0 := a - (mu-1)/(8*a) - (4*(mu-1)*(7*mu-31))/(3*math.Pow(8*a, 3))
	x0 -= 32 * ((mu - 1) * (83*mu*mu - 982*mu + 3779)) / (15 * math.Pow(8*a, 5))
	x0 -= 64 * ((mu - 1) * (6949*mu*mu*mu - 153855*mu*mu + 1585743*mu - 6277237)) / (105 * math.Pow(8*a, 7))
	return x0
}

func Zeros(v, m int, zeros [][]float64) [][]float64 {
	// v -- max degree - 1
	// m -- max zero no.

	// table for degree = 0
	current := make([]float64, m+v-1)
	var len_zeros_0 int
	if len(zeros) == 0 {
		len_zeros_0 = 0
	} else {
		len_zeros_0 = len(zeros[0])
	}
	for i := 0; i < len_zeros_0; i++ {
		current[i] = zeros[0][i]
	}
	for i := len_zeros_0; i < m+v-1; i++ {
		current[i] = zero_newton(0, bessel_guess(0, i+1))
	}
	// not precise enough, use j_v,1 < j_v+1,1 < j_v,2 < j_v+1,2 ...

	len_zeros := len(zeros)
	var beg int
	ans := make([][]float64, v)
	ans[0] = current
	for i := 1; i < v; i++ {
		hold := make([]float64, m+v-i-1)
		if i < len_zeros {
			beg = len(zeros[i])
		} else {
			beg = 0
		}
		for j := 0; j < beg; j++ {
			hold[j] = zeros[i][j]
		}
		for j := beg + 1; j < m+v-i; j++ {
			x0 := current[j-1]
			zero_hold := zero_newton(i, x0)
			for zero_hold > current[j] || zero_hold < current[j-1] {
				x0 += 1
				zero_hold = zero_newton(i, x0)
			}
			hold[j-1] = zero_newton(i, x0)
		}
		current = hold
		ans[i] = current
	}
	return ans
}
