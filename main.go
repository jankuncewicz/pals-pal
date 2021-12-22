package main

import (
	"fmt"
	"math"
)

const acc = 0.01

func zero_newton(n int, x0 float64) float64 {
	err := 1.0
	x := 0.1
	for math.Abs(err) > acc {
		b_hold := math.Jn(n, x0)
		x = x0 - b_hold/(math.Jn(n-1, x0)-(float64(n)/x0)*b_hold)
		err = x0 - x
		x0 = x
		//fmt.Println(x)
	}
	return x
}

func bessel_zero(v, m int) {
	// initial guess
	mu := float64(4 * v * v)
	a := (float64(m) + (1.0/2)*float64(v) - (1.0 / 4)) * math.Pi
	x0 := a - (mu-1)/(8*a) - (4*(mu-1)*(7*mu-31))/(3*math.Pow(8*a, 3))
	x0 -= 32 * ((mu - 1) * (83*mu*mu - 982*mu + 3779)) / (15 * math.Pow(8*a, 5))
	x0 -= 64 * ((mu - 1) * (6949*mu*mu*mu - 153855*mu*mu + 1585743*mu - 6277237)) / (105 * math.Pow(8*a, 7))

	// not precise enough, use j_v,1 < j_v+1,1 < j_v,2 < j_v+1,2 ...
	// because it is precise for small v but for large m

	fmt.Println(zero_newton(v, x0))
}

func main() {

}
