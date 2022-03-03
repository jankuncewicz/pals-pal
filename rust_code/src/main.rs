#[path = "ete/tau.rs"]
mod tau;
#[path = "bessel/bessel_zero.rs"]
mod zero;
#[path = "bessel/bessel_integral.rs"]
mod int;

fn main() {
	println!("{}", tau::tau(100, 100, 1.8/2.0, 0.193, 300.0));
}