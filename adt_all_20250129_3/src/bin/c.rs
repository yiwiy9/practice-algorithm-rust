use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5*n],
    }

    x.sort();

    println!(
        "{}",
        x.iter().skip(n).take(3 * n).sum::<usize>() as f64 / (3.0 * n as f64)
    );
}
