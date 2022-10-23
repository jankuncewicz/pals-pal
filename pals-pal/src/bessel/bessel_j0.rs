// calculation J_0(x) taken from GSL bessel_J0.c

#[path = "cheb.rs"]
mod cheb;
#[path = "bessel_amp_phase.rs"]
mod bessel_amp_phase;
#[path = "bessel_sf.rs"]
mod bessel_sf;

const BJ0_DATA: [f64; 13] = [
   0.100254161968939137, 
  -0.665223007764405132, 
   0.248983703498281314, 
  -0.0332527231700357697,
   0.0023114179304694015,
  -0.0000991127741995080,
   0.0000028916708643998,
  -0.0000000612108586630,
   0.0000000009838650793,
  -0.0000000000124235515,
   0.0000000000001265433,
  -0.0000000000000010619,
   0.0000000000000000074,
];

pub fn j0(x: f64) -> f64 {
	let y = x.abs();
	if y <= 4.0 {
		return cheb::cheb_eval(0.125*y*y - 1.0, &BJ0_DATA, -1.0, 1.0);
	}
	else {
		let z = 32.0/(y*y) - 1.0;
		let ca = cheb::cheb_eval(z, &bessel_amp_phase::BM0_DATA, -1.0, 1.0);
		let ct = cheb::cheb_eval(z, &bessel_amp_phase::BTH0_DATA, -1.0, 1.0);
		let cp = bessel_sf::cos_pi4(y, ct/y);
		let ampl = (0.75 + ca) / y.sqrt();
		return ampl * cp;
	}
}