package ete

import "math"

func Enm(zero, R, delta, T float64) float64 {
	// c = hbar^2/4me*k
	const c = 221.068
	return -c * math.Pow(zero/(R+delta), 2) / T
}

// 1.1121 * 10^(-68) J^2*s^2 / 3.64375348 * 10^(-30) kg
// 0.3052133 * 10^(-38) J^2*s^2/kg
// this/k = 0.3052133 * 10^(-38) J^2*s^2/kg / 1.381 * 10^(-23) J/K =
// 0.22107 * 10^(-15) J*K*s^2/kg = kg*(m^2/s^2)*K*s^2 = kg*m^2*K
