mod bessel_j0;
mod bessel_j1;
#[path = "bessel_expansion.rs"]
mod exp;

pub fn jn(v: i32, y: f64) -> f64 {
	let mut sign = 1.0;
	let mut n = v;
	let mut x = y;
	if v < 0 {
		n = -n;
		if v % 2 == 1 {sign = -sign;}
	}
	if x < 0.0 {
		x = -x;
		if v % 2 == 1 {sign = -sign;}
	} 
	if n == 0 {
		return sign * bessel_j0::j0(x);
	}
	else if n == 1{
		return sign * bessel_j1::j1(x);
	}
	else {
		// First I will only try Hankel asymptotic expansion	
		if x > 1000.0 {	
			return exp::hankel(v as f64, x);
		}
		// recurence relation that i will examine later
		else{
			let ans;
			let mut ratio = 0.0;
			let mut sgn = 0.0;
			exp::bessel_j_cf1(n as f64, x, &mut ratio, &mut sgn);
			/* backward recurrence */
			let mut jkp1 = exp::GSL_SQRT_DBL_MIN * ratio;
			let mut jk   = exp::GSL_SQRT_DBL_MIN;
			let mut jkm1;
			for k in (1..n+1).rev() {
			  jkm1 = 2.0*(k as f64)/x * jk - jkp1;
			  jkp1 = jk;
			  jk   = jkm1;
			}
			if jkp1.abs() > jk.abs() {
				let b1 = bessel_j1::j1(x);
				ans = b1/jkp1*exp::GSL_SQRT_DBL_MIN;
			}
			else {
				let b0 = bessel_j0::j0(x);
				ans = b0/jk * exp::GSL_SQRT_DBL_MIN;
			}
			return sign * ans;
    	}
  	}
}