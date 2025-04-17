use proconio::input;

/// https://atcoder.jp/contests/abc377/tasks/abc377_
/// https://atcoder.jp/contests/abc377/editorial/11226
///
/// r(=m)が全探索可能であるというポイントに注目する
/// まず、重要な事実として整数の組 (l,r) が条件を満たすなら (l+1,r) も条件を満たします。
/// => r を固定した時に、l は d_r以上r以下の整数を取れるというd_rが存在する
/// => あとは、d_r-1からd_rへは漸化式を使って求めることができる
fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n],
    }

    let mut d = vec![1; m + 1];
    for &(l, r) in &lr {
        // あるr対して、MAXのlを記録する（他は内包される）
        d[r] = d[r].max(l + 1);
    }

    // 漸化式でdを更新する
    for r in 1..=m {
        // d[r]はd[r-1]とd[r]の最大値を取る
        // 各rに対する条件を満たす最小の左端を更新
        d[r] = d[r].max(d[r - 1]);
    }

    let mut ans = 0;
    for r in 1..=m {
        ans += r + 1 - d[r];
    }

    println!("{}", ans);
}
