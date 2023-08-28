use proconio::{input, marker::Usize1};

/**
 * Wが和じゃないので普通に貪欲でいけるみたい
 * => 価値の大きな荷物から順に、その荷物を入れることができる大きさ最小の箱に入れる
 * https://atcoder.jp/contests/abc195/editorial/846
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut wv: [(usize, i32); n],
        x: [usize; m],
        lr: [(Usize1, Usize1); q],
    }
    wv.sort_by(|&a, &b| b.0.cmp(&a.0));

    for &(l, r) in &lr {
        let mut cur_x = x
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < l || r < *i)
            .map(|(_, &x_i)| x_i)
            .collect::<Vec<_>>();

        cur_x.sort_by(|&a, &b| b.cmp(&a));
        let cur_m = cur_x.len();

        let mut dp = vec![vec![0; cur_m + 1]; n + 1];
        for i in 0..n {
            // 選ばない遷移のことを考えて、MAX値は含む必要がある！！
            for j in 0..=cur_m {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                if j != cur_m && wv[i].0 <= cur_x[j] {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + wv[i].1);
                }
            }
        }
        println!("{}", dp[n].iter().max().unwrap());
    }
}
