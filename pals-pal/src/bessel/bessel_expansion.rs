pub const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;

pub fn hankel(nu: f64, x: f64) -> f64 {
	let mu = 4.0*nu*nu;
	let chi = x - (0.5*nu + 0.25)*std::f64::consts::PI;
	
	let mut p: f64 = 0.0;
	let mut q: f64 = 0.0; 
	
	let mut k = 0.0;
	let mut t = 1.0;
	
	let mut convp;
	let mut convq;

	while k < 1000.0 {
		if k == 0.0 {t = 1.0;}
		else {t *= -(mu - (2.0*k-1.0)*(2.0*k-1.0)) / (k*(8.0*x));}
		convp = t.abs() < GSL_DBL_EPSILON*p.abs();
		p += t;

		k += 1.0;

		t *= (mu - (2.0*k-1.0)*(2.0*k-1.0)) / (k*(8.0*x));
		convq = t.abs() < GSL_DBL_EPSILON*q.abs();
		q += t;

		if convp && convq && k > (nu/2.0) {break;}

		k += 1.0;
	}	
	return (2.0/(std::f64::consts::PI*x)).sqrt() * (chi.cos()*p - chi.sin()*q);
}

pub const GSL_SQRT_DBL_MIN: f64 = 1.4916681462400413e-154;
pub const GSL_SQRT_DBL_MAX: f64 = 1.3407807929942596e+154;

pub fn bessel_j_cf1(nu: f64, x: f64, ratio: &mut f64, sgn: &mut f64) {
	const RECUR_BIG: f64 = GSL_SQRT_DBL_MAX;
	const RECUR_SMALL: f64 = GSL_SQRT_DBL_MIN;
	let maxiter = 10000.0;
	let mut n = 1.0;
	let mut Anm2 = 1.0;	
	let mut Bnm2 = 0.0;
	let mut Anm1 = 0.0;
	let mut Bnm1 = 1.0;
	let a1 = x/(2.0*(nu+1.0));
	let mut An = Anm1 + a1*Anm2;
	let mut Bn = Bnm1 + a1*Bnm2;
	let mut an;
	let mut f_n = An/Bn;
	let mut dn = a1;
	let mut s  = 1.0;

	while n < maxiter {
		let old_fn;
		let del;
		n += 1.0;
		Anm2 = Anm1;
		Bnm2 = Bnm1;
		Anm1 = An;
		Bnm1 = Bn;
		an = -x*x/(4.0*(nu+n-1.0)*(nu+n));
		An = Anm1 + an*Anm2;
		Bn = Bnm1 + an*Bnm2;
		if An.abs() > RECUR_BIG || Bn.abs() > RECUR_BIG {
   		 	An /= RECUR_BIG;
		 	Bn /= RECUR_BIG;
			Anm1 /= RECUR_BIG;
			Bnm1 /= RECUR_BIG;
			Anm2 /= RECUR_BIG;
		} 
		else if An.abs() < RECUR_SMALL || Bn.abs() < RECUR_SMALL {
			An /= RECUR_SMALL;
			Bn /= RECUR_SMALL;
			Anm1 /= RECUR_SMALL;
			Bnm1 /= RECUR_SMALL;
			Anm2 /= RECUR_SMALL;
			Bnm2 /= RECUR_SMALL;
		}
		old_fn = f_n;
		f_n = An/Bn;
		del = old_fn/f_n;

		dn = 1.0 / (2.0*(nu+n)/x - dn);
		if dn < 0.0 {s = -s;}

		if (del - 1.0).abs() < 2.0*GSL_DBL_EPSILON {break;}
	}
	*ratio = f_n;
	*sgn = s;
}