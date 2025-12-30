use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut cg: [(i64,Usize1); n],
    }

    cg.sort_by(|a, b| b.cmp(a));

    let mut genre_prices = vec![vec![-1; k + 1]; 10];
    for genre in 0..10 {
        let mut cnt = 0;
        let mut sum = 0;
        genre_prices[genre][0] = 0;
        for &(c, g) in &cg {
            if g == genre {
                cnt += 1;
                sum += c;
                genre_prices[genre][cnt as usize] = sum + cnt * (cnt - 1);
                if cnt as usize == k {
                    break;
                }
            }
        }
    }

    let mut dp = vec![0; k + 1];
    for genre in 0..10 {
        let mut next_dp = vec![0; k + 1];
        for i in 0..=k {
            for j in 0..=k - i {
                if genre_prices[genre][j] == -1 {
                    break;
                }
                next_dp[i + j] = next_dp[i + j].max(dp[i] + genre_prices[genre][j]);
            }
        }
        dp = next_dp;
    }

    println!("{}", dp[k]);
}
