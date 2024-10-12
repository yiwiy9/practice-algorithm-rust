use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.iter()
            .filter(|&&a_i| a_i % k == 0)
            .map(|&a_i| a_i / k)
            .join(" ")
    );
}
