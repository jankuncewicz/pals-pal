#[path = "bessel_jn.rs"]
mod bessel;

const X: [f64; 10] = [
	0.07652652113349733375,
	0.22778585114164507808,
	0.37370608871541956067,
	0.51086700195082709800,
	0.63605368072651502545,
	0.74633190646015079261,
	0.83911697182221882339,
	0.91223442825132590586,
	0.96397192727791379126,
	0.99312859918509492478,
];

const W: [f64; 10] = [
	0.15275338713072585069,
	0.14917298647260374678,
	0.14209610931838205132,
	0.13168863844917662689,
	0.11819453196151841731,
	0.10193011981724043503,
	0.08327674157670474872,
	0.06267204833410906357,
	0.04060142980038694133,
	0.01761400713915211831,
];

pub fn gauss(start: f64, end: f64, n: i32) -> f64{
	let mut ans = 0.0;
	let a = (start + end) / 2.0;
	let b = (end - start) / 2.0;
	for i in 0..W.len() {
		ans += W[i] * bessel::jn(n, b*X[i] + a).powi(2) * (b*X[i] + a);
		ans += W[i] * bessel::jn(n, -b*X[i] + a).powi(2) * (-b*X[i] + a);
	}
	if ans.is_nan() {
		return 0.0;
	}
	return b * ans;
}

//pub fn int_bessel(start: f64, end: f64, n: i32) -> f64 {
//	let mut ans = 0.0;
//	let mut now = start;
//	let mut startm = start;
//	loop {
//		now += 20.0;
//		if now >= end {
//			ans += gauss(startm, end, n);
//			break;
//		}
//		ans += gauss(startm, now, n);
//		startm = now;
//	}
//	return ans;
//}