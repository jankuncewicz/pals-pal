mod utils;

use wasm_bindgen::prelude::*;

#[path = "ete/approx_r.rs"]
mod r;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut ZEROS: Vec<Vec<f64>> = Vec::new();
static mut INTS: Vec<Vec<f64>> = Vec::new();

#[wasm_bindgen]
pub fn main(tau: f64, delta: f64, t: f64) -> f64 {
	unsafe{
		return r::approx(tau, 100, 100, delta, t, 0.000001, &mut ZEROS, &mut INTS);
	}
	//println!("{:?}", zeros);
}
