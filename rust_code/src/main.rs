extern crate rgsl;

fn zero_newton(n: i32, x0: f64) -> f64 {
    let acc: f64 = 0.001;
    let mut err: f64 = 1.0;
    
    let mut x: f64 = 0.0;
    let mut hold: f64;
    let mut hold_prev: f64;
    
    let nf = n as f64;
    let mut x0_mut = x0;

    while err.abs() > acc {
        hold = rgsl::bessel::Jn(n, x0_mut);
        if n == 0 {
            hold_prev =  -rgsl::bessel::Jn(1, x0_mut)
        }
        else {
            hold_prev =  rgsl::bessel::Jn(n-1, x0_mut)
        }
        x = x0_mut - hold/(hold_prev - (nf/x0_mut)*hold);
        err = x0_mut - x;
        x0_mut = x;
    }
    return x;
}

fn bessel_guess(v: i32, m: i32) -> f64 {
    let vf = v as f64;
    let mf = m as f64;
	let mu = 4.0 * vf * vf;
    let pi = std::f64::consts::PI;
	let a = (mf + 0.5*vf - 0.25) * pi;
	let x0 = a - (mu-1.0)/(8.0*a) 
        - 4.0*(mu-1.0)*(7.0*mu-31.0)/(3.0*(8.0*a).powi(3))
	    - 32.0 * (mu - 1.0) * (83.0*mu*mu - 982.0*mu + 3779.0) / (15.0 *(8.0*a).powi(5))
	    - 64.0 * (mu - 1.0) * (6949.0*mu*mu*mu - 153855.0*mu*mu + 1585743.0*mu - 6277237.0) / (105.0 * (8.0*a).powi(7));
    return x0;
}

fn zeros(v: i32, m: i32) -> Vec<Vec<f64>> {
    let mut current: Vec<f64> = Vec::new();
    for i in 0..m+v-1 {
        current.push(zero_newton(0, bessel_guess(0, i+1)));
    }
    let mut ans: Vec<Vec<f64>> = vec![];
    ans.push(current);
    return ans;
}

fn main() {
    //for i in 1..100 {
    //    print!("{} ", rgsl::bessel::zero_J0(i));
    //}
    let zeros_b = zeros(1, 1000);
    println!("{}", zeros_b[0].len());
}