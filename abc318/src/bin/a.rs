use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }

    if n < m {
        println!("{}", 0);
        return;
    }

    println!("{}", 1 + (n - m) / p);
}
