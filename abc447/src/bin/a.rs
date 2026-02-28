use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    println!("{}", if (n + 1) / 2 >= m { "Yes" } else { "No" });
}
