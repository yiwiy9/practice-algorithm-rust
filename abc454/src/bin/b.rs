use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Usize1; n],
    }

    let mut f_sorted = f.clone();
    f_sorted.sort();
    f_sorted.dedup();
    println!("{}", if f_sorted.len() == n { "Yes" } else { "No" });
    println!("{}", if f_sorted.len() == m { "Yes" } else { "No" });
}
