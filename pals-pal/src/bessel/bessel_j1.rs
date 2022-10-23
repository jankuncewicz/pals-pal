// calculation J_1(x) taken from GSL bessel_J1.c

#[path = "cheb.rs"]
mod cheb;
#[path = "bessel_amp_phase.rs"]
mod bessel_amp_phase;
#[path = "bessel_sf.rs"]
mod bessel_sf;

const BJ1_DATA: [f64; 12] = [
  -0.11726141513332787,
  -0.25361521830790640,
   0.050127080984469569,
  -0.004631514809625081,
   0.000247996229415914,
  -0.000008678948686278,
   0.000000214293917143,
  -0.000000003936093079,
   0.000000000055911823,
  -0.000000000000632761,
   0.000000000000005840,
  -0.000000000000000044,
];

pub fn j1(x: f64) -> f64 {
	let y = x.abs();
	if y <= 4.0 {
		return x * (0.25 + cheb::cheb_eval(0.125*y*y - 1.0, &BJ1_DATA, -1.0, 1.0));
	}
	else {
		let z = 32.0/(y*y) - 1.0;
		let ca = cheb::cheb_eval(z, &bessel_amp_phase::BM1_DATA, -1.0, 1.0);
		let ct = cheb::cheb_eval(z, &bessel_amp_phase::BTH1_DATA, -1.0, 1.0);
		let sp = bessel_sf::sin_pi4(y, ct/y);
		let ampl = (0.75 + ca) / y.sqrt();
		if x < 0.0 {
			return -ampl * sp;
		}
		else {
			return ampl * sp;
		}
	}
}