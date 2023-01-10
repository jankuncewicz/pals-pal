mod utils;

use wasm_bindgen::prelude::*;
use js_sys::{Float64Array, Map};
use console_error_panic_hook::set_once;

#[path = "ete/approx_r.rs"]
mod r;

#[path = "ete/tau.rs"]
mod tau;

#[path = "plotting/plot.rs"]
mod func_plot;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut ZEROS: Vec<Vec<f64>> = Vec::new();
static mut INTS: Vec<Vec<f64>> = Vec::new();

#[wasm_bindgen]
pub fn calculate_r(tau: f64, delta: f64, t: f64) -> f64 {
	unsafe{
        set_once();
        console_log!("{}", ZEROS.len());
		let ans = r::approx(tau, 100, 100, delta, t, 0.000001, &mut ZEROS, &mut INTS);
        console_log!("{}", ZEROS.len());
        console_log!("{}", ans*2.0);
        return ans;
	}
	//println!("{:?}", zeros);
}

#[wasm_bindgen]
pub fn calculate_tau(r: f64, delta: f64, t: f64) -> f64 {
    unsafe{
        console_log!("zeros len: {}", ZEROS.len());
        if ZEROS.len() != 0 {
            console_log!("zeros[0] len: {}", ZEROS[0].len());
        }
        return tau::tau(200, 200, r, delta, t, 0.000001, &mut ZEROS, &mut INTS);
    }
}

#[wasm_bindgen]
pub fn calculate_array(arr: &Float64Array, delta: f64, t: f64) {
    unsafe{
        let data: Vec<f64> = arr.to_vec();
        let test: Vec<f64> = data.iter().map(|x| {
            let h: f64 = r::approx(*x, 100, 100, delta, t, 0.000001, &mut ZEROS, &mut INTS);
            console_log!("{}, {}", *x, h);
            return h;
        }).collect();
        console_log!("Finished!");
        arr.copy_from(&test);
    }
}

#[wasm_bindgen]
pub fn test(t: &Map) {
    t.for_each(&mut |val, key| {
        console_log!("{:?} {:?}", val, key);
    });
}

#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
        let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
