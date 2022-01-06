package ete

func Lambda_nm(pnm float64) float64 {
	const lam_T = 0.00704
	const lam_S = 7.9895
	const lam_b = 0.25*lam_S + 0.75*lam_T
	// TODO: this doesn't include preassure
	// TODO: which is better?
	//return lam_b*pnm + lam_T*(1-pnm)
	return lam_b*pnm + lam_T
}
