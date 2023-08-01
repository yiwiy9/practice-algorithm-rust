use proconio::input;

const MAX_T: usize = 100_000;

fn main() {
    input! {
        n: usize,
        txa: [(usize,usize,i64); n],
    }

    let mut sizes = vec![vec![0; 5]; MAX_T + 1];
    for &(t, x, a) in &txa {
        sizes[t][x] = a;
    }

    let mut dp = vec![vec![-1; 5]; MAX_T + 1];
    dp[0][0] = 0;
    for t in 0..MAX_T {
        for x in 0..5 {
            if dp[t][x] == -1 {
                continue;
            }

            dp[t + 1][x] = dp[t + 1][x].max(dp[t][x] + sizes[t + 1][x]);
            if x > 0 {
                dp[t + 1][x - 1] = dp[t + 1][x - 1].max(dp[t][x] + sizes[t + 1][x - 1]);
            }
            if x < 4 {
                dp[t + 1][x + 1] = dp[t + 1][x + 1].max(dp[t][x] + sizes[t + 1][x + 1]);
            }
        }
    }

    println!("{}", dp[MAX_T].iter().max().unwrap());
}
