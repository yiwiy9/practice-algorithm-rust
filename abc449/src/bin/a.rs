use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        d: f64,
    }

    println!("{}", d * d * PI / 4.0);
}
