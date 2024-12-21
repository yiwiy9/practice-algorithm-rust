use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }

    let d_cos = d.to_radians().cos();
    let d_sin = d.to_radians().sin();

    let new_a = a * d_cos - b * d_sin;
    let new_b = a * d_sin + b * d_cos;

    println!("{:.10} {:.10}", new_a, new_b);
}
