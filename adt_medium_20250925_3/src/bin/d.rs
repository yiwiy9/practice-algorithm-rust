use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let c = (a * a + b * b).sqrt();
    println!("{:.10} {:.10}", a / c, b / c);
}
