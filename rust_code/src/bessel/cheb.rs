// chebyshev expansion
// taken from GSL cheb_eval.c

pub fn cheb_eval(x: f64, c: &[f64], a: f64, b: f64) -> f64 {
	let mut d  = 0.0;
	let mut dd = 0.0;

	let y  = (2.0*x - a - b) / (b - a);
	let y2 = 2.0 * y;

	for i in (1..c.len()).rev() {
		let temp = d;
		d = y2*d - dd + c[i];
		dd = temp;
	}

	d = y*d - dd + 0.5*c[0];
	return d;
}
//0.00001326828430100585
//0.000013268284301218292