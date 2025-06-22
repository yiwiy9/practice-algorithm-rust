use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n-1],
    }

    let mut d_sum = vec![0; n];
    for i in 0..n - 1 {
        d_sum[i + 1] = d_sum[i] + d[i];
    }

    for i in 0..n - 1 {
        let mut ans = vec![];
        for j in i + 1..n {
            ans.push(d_sum[j] - d_sum[i]);
        }

        println!("{}", ans.iter().join(" "));
    }
}
