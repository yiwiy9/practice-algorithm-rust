use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e_vec: [f64; q],
    }

    for &e in &e_vec {
        let (e8_y, e8_z) = e8_coordinate(e, t, l);
        let radians = f64::atan2(e8_z, f64::sqrt(x * x + (y - e8_y) * (y - e8_y)));
        println!("{:.8}", radians.to_degrees());
    }
}

fn e8_coordinate(e: f64, t: f64, l: f64) -> (f64, f64) {
    (
        (l / 2.) * f64::cos(3. * PI / 2. - 2. * PI * e / t),
        (l / 2.) * f64::sin(3. * PI / 2. - 2. * PI * e / t) + l / 2.,
    )
}
