use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let sum = t.iter().sum::<usize>();

    let mut dp = vec![vec![false; sum + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=sum {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + t[i] <= sum {
                    dp[i + 1][j + t[i]] = true;
                }
            }
        }
    }

    println!(
        "{}",
        dp[n]
            .iter()
            .enumerate()
            .skip((t.iter().sum::<usize>() + 1) / 2)
            .find_or_first(|(_t, &v)| v)
            .unwrap()
            .0
    );
}
