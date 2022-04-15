#[path = "../bessel/bessel_integral.rs"]
mod int;
#[path = "../bessel/bessel_zero.rs"]
mod zero;

fn energy(zero: f64, r: f64, delta: f64, t: f64) -> f64 {
	const C: f64 = 221.068;
	return -C * (zero/(r+delta)).powi(2) / t;
}

fn pnm(m: i32, r: f64, delta: f64, zero : f64, inte: f64) -> f64 {
	return int::gauss(zero*(r/(r+delta)), zero, m)	/ inte;
}

fn lambda_nm(pnm: f64) -> f64 {
	const LAM_T: f64 = 0.00704;
	const LAM_S: f64 = 7.9895;
	const LAM_B: f64 = 0.25*LAM_S + 0.75*LAM_T;
	// TODO: this doesn't include preassure
	// TODO: which is better?
	return LAM_B*pnm + LAM_T*(1.0-pnm);
	//return LAM_B*pnm + LAM_T;
}

fn g(m: usize) -> f64{
	if m == 0 {
		return 1.0;
	}
	return 2.0;
}

// testing without precalculated zeros
pub fn tau(n: i32, m: i32, r: f64, delta: f64, t: f64, res: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {

	let mut sum1 = 0.0;
	let mut sum2 = 0.0;
	
	for i in 0..(n as usize) {
		if zeros.len() <= i {
			zeros.push(Vec::new());
			ints.push(Vec::new());
		}
		for j in 0..(m as usize) {
			if zeros[i].len() <= j{
				zeros[i].push(zero::zero_jnu(i as f64, (j + 1) as u32));
				if j == 0 {
					ints[i].push(int::gauss(0.0, zeros[i][0], i as i32));
				}
				else {
					let prev = ints[i][j-1];
					ints[i].push(prev + int::gauss(zeros[i][j-1], zeros[i][j], i as i32));
				}
			}
			let z = zero::zero_jnu(i as f64, (j + 1) as u32);
			let e = energy(zeros[i][j], r, delta, t).exp();
			let prev = sum1/sum2;
			sum1 += g(i) * e;
			sum2 += g(i) * e * lambda_nm(pnm(i as i32, r, delta, z, ints[i][j]));
			if (sum1/sum2 - prev).abs() <= res && (i != 0 || j != 0){
				break;
			}
		}
	}
	return sum1/sum2;
}