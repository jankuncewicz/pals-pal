mod tau;

fn zero_newton(x0: f64, tau: f64, n: i32, m: i32, delta: f64, t: f64, res: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {

	let mut err: f64 = 1.0;
	const d: f64 = 0.001;
	let mut x = x0;
	let mut x2;

	for _i in 0..1000 {
		if err.abs() < res {
			break;
		}
		let f1 = tau::tau(n, m, x, delta, t, res, zeros, ints) - tau;
		let f2 = tau::tau(n, m, x+d*x, delta, t, res, zeros, ints) - tau;

		let deriv = (f2 - f1) / (d * x);
		x2 = x - f1/deriv;
		x2 = x2.abs().rem_euclid(140.0);
		err = x - x2;
		x = x2;
	}
	return x;
}

pub fn approx(tau: f64, n: i32, m: i32, delta: f64, t: f64, res: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {

	let guess = 0.25 * ((39480499.0*tau)/(1248483.0*(140.0-tau).abs())).powf(2.0/3.0);
	return zero_newton(guess, tau, n, m, delta, t, res, zeros, ints);
}