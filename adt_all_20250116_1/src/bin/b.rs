use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n == 1 || n > 4 { "Yes" } else { "No" });
}
