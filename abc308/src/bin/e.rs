use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut dp = vec![vec![vec![0; 1 << 3]; 4]; n + 1];
    dp[0][0][0] = 1;

    for (i, c) in s.iter().enumerate() {
        let before_j = match *c {
            'M' => 0,
            'E' => 1,
            'X' => 2,
            _ => unreachable!(),
        };
        for j in 0..4 {
            for k in 0..(1 << 3) {
                dp[i + 1][j][k] += dp[i][j][k];
                if j == before_j {
                    dp[i + 1][j + 1][k | (1 << a[i])] += dp[i][j][k];
                }
            }
        }
    }

    println!(
        "{}",
        dp[n][3]
            .iter()
            .enumerate()
            .map(|(k, cnt)| {
                let mex: usize;
                let mut i = 0;
                loop {
                    if k & (1 << i) == 0 {
                        mex = i;
                        break;
                    }
                    i += 1;
                }
                mex * cnt
            })
            .sum::<usize>()
    );
}
