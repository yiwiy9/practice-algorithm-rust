use proconio::input;

fn main() {
    input! {
        h: f64,
    }

    println!("{}", (h * (12800000.0 + h)).sqrt());
}
