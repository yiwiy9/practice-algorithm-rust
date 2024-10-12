use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![]; n];
    for i in 0..n {
        for j in 0..=i {
            if j == 0 || j == i {
                a[i].push(1);
            } else {
                let a_ij = a[i - 1][j - 1] + a[i - 1][j];
                a[i].push(a_ij);
            }
        }
    }

    println!("{}", a.iter().map(|a_i| a_i.iter().join(" ")).join("\n"));
}
