mod tau;

use argmin::solver::brent::BrentRoot;
use argmin::core::{Executor, CostFunction};
use lazy_static::lazy_static;
use std::sync::Mutex;

fn zero_brent(x0: f64, tau: f64, n: usize, m: usize, delta: f64, t: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {

	lazy_static! {
		static ref G_ZEROS : Mutex<Vec<Vec<f64>>> = Mutex::new(vec![]);
		static ref G_INTS : Mutex<Vec<Vec<f64>>> = Mutex::new(vec![]);
	}

	G_ZEROS.lock().unwrap().clone_from(&zeros);
	G_INTS.lock().unwrap().clone_from(&ints);

	struct Problem {
		tau: f64,
		n: usize,
		m: usize, 
		delta: f64, 
		t: f64, 
	}

	impl CostFunction for Problem {
		type Param = f64;
		type Output = f64;

		fn cost(&self, r: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
			let mut zeros = G_ZEROS.lock().unwrap();
			let mut ints = G_INTS.lock().unwrap();
			return Ok(tau::tau(self.n, self.m, *r, self.delta, self.t, &mut zeros, &mut ints) - self.tau);	
		}
	}
	

	let problem = Problem{tau, n, m, delta, t};
	let solver = BrentRoot::new(0.0, 142.0, 1e-5);
	let exec = Executor::new(problem, solver).configure(|state| state.param(x0).max_iters(200)).run();

	zeros.clone_from(&G_ZEROS.lock().unwrap());
	ints.clone_from(&G_INTS.lock().unwrap());

	return exec.unwrap().state().best_param.unwrap();
}

fn zero_secant(x0: f64, tau: f64, n: usize, m: usize, delta: f64, t: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {

	const d: f64 = 0.001;
	let mut x = x0;
	let mut x2 = x0;
	const RES: f64 = 1e-5;

	let mut f1 = tau::tau(n, m, x, delta, t, zeros, ints) - tau;
	
	for _i in 0..1000 {
		if f1.abs() < RES {
			break;
		}

		let f2 = tau::tau(n, m, x+d*x, delta, t, zeros, ints) - tau;

		let deriv = (f2 - f1) / (d * x);

		if deriv < 1e-1 {
			// pass it to brent's method
			return zero_brent(x, tau, n, m, delta, t, zeros, ints);
		}
		else {
			x2 = x - f1/deriv;
			x2 = x2.abs().rem_euclid(143.0);
		}

		let fx2 = tau::tau(n, m, x2, delta, t, zeros, ints) - tau;

		x = x2;
		f1 = fx2;
	}
	return x;
	
}

pub fn approx(tau: f64, n: usize, m: usize, delta: f64, t: f64,
	zeros: &mut Vec<Vec<f64>>, ints: &mut Vec<Vec<f64>>) -> f64 {
	let guess = 0.25 * ((39480499.0*tau)/(1248483.0*(140.0-tau).abs())).powf(2.0/3.0);
	return zero_secant(guess, tau, n, m, delta, t, zeros, ints);
}
