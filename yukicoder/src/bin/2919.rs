use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    println!("{}", if m + k <= n { "Yes" } else { "No" });
}
