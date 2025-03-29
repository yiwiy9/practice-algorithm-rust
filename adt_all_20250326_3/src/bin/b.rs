use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: f64,
        x: usize,
    }

    if x <= a {
        println!("{}", 1.0);
    } else if x <= b {
        println!("{}", c / (b - a) as f64);
    } else {
        println!("{}", 0.0);
    }
}
