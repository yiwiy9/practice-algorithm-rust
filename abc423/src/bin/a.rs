use proconio::input;

fn main() {
    input! {
        x: usize,
        c: usize,
    }

    println!("{}", (1000 * x / (1000 + c)) / 1000 * 1000);
}
