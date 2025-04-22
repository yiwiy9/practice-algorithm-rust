use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc402/tasks/abc402_d
/// https://atcoder.jp/contests/abc402/editorial/12758
///
/// 全体の組み合わせ - 平行の組み合わせ = 答え
///
/// こういう問題は実験して、法則を見つけるしかない
/// => (Ai + Bi) % N が同じものが平行となる
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut cnt = vec![0; n];
    for &(a, b) in &ab {
        cnt[(a + b) % n] += 1;
    }

    let mut ans = m * (m - 1) / 2;
    for i in 0..n {
        if cnt[i] > 1 {
            ans -= cnt[i] * (cnt[i] - 1) / 2;
        }
    }

    println!("{}", ans);
}
