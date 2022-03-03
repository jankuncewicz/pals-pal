pub fn cos_pi4(y: f64, eps: f64) -> f64 {
	return (y - std::f64::consts::PI/4.0 + eps).cos();
}

pub fn sin_pi4(y: f64, eps: f64) -> f64 {
	return (y - std::f64::consts::PI/4.0 + eps).sin();
}