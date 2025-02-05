use proconio::input;

/// https://qiita.com/pinokions009/items/1e98252718eeeeb5c9ab
/// 桁DPの痒いところに手が届く解説
///
/// https://atcoder.jp/contests/abc387/tasks/abc387_c
/// https://atcoder.jp/contests/abc387/editorial/11870
/// dp[i][j][k][l] := i桁目まで決めて、0でない最初の数字がjであり、xより小さいことが確定しているかどうか（k）と0以外の数字が出現しているかどうか（l）
fn main() {
    input! {
        l: usize,
        r: usize
    }

    println!("{}", f(r) - f(l - 1));
}

fn f(x: usize) -> usize {
    let x = x.to_string().chars().collect::<Vec<_>>();

    let mut dp = vec![vec![vec![0_usize; 2]; 2]; 10];
    dp[0][1][0] = 1;
    let first_digit = x[0] as usize - '0' as usize;
    for j in 1..first_digit {
        dp[j][1][1] = 1;
    }
    dp[first_digit][0][1] = 1;

    for i in 1..x.len() {
        let mut next_dp = vec![vec![vec![0; 2]; 2]; 10];
        let x_digit = x[i] as usize - '0' as usize;
        for j in 0..10 {
            for k in 0..2 {
                for l in 0..2 {
                    for d in 0..10 {
                        if l == 1 && j <= d {
                            // ヘビ数の定義をみたす（＝先頭の要素より小さい）かどうか
                            continue;
                        }
                        if k == 0 && d > x_digit {
                            // x 以下であるか
                            continue;
                        }

                        let next_k = if k == 1 || d < x_digit { 1 } else { 0 };
                        let next_l = if l == 1 || d > 0 { 1 } else { 0 };
                        let next_j = if l == 1 { j } else { d };
                        next_dp[next_j][next_k][next_l] += dp[j][k][l];
                    }
                }
            }
        }
        dp = next_dp;
    }

    dp.iter()
        .map(|dp_kl| {
            dp_kl
                .iter()
                .map(|dp_l| {
                    dp_l.iter()
                        .enumerate()
                        .filter(|&(i, _)| i == 1)
                        .map(|(_, &v)| v)
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}
