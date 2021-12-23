package main

import (
	"fmt"
	"pals_pal/bessel_zeros"
)

func main() {
	test := bessel_zeros.Zeros(100, 100)
	fmt.Println(test[99][0])
}
