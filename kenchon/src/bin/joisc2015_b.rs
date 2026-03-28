use proconio::{input, marker::Chars};

/// https://www2.ioi-jp.org/camp/2015/2015-sp-tasks/2015-sp-d1.pdf
/// https://www2.ioi-jp.org/camp/2015/2015-sp-tasks/2015-sp-d1-enjoi-review.pdf
///
/// dp[start] := 現在見ている level-1 において、
/// t[start..start+len) を level-1 JOI 列にする最小変更回数
///
/// 遷移先の ndp[start] は、level JOI 列の定義
/// = [J を len 個] [O を len 個] [I を len 個] [level-1 JOI 列]
/// をそのまま使って計算する
fn main() {
    input! {
        k: usize,
        s: Chars,
    }

    let n = s.len();
    let mut t = s.clone();
    t.extend_from_slice(&s); // 円環を直線として扱うために2倍化

    let mut pref = vec![vec![0usize; 2 * n + 1]; 3];
    for i in 0..2 * n {
        for c in 0..3 {
            pref[c][i + 1] = pref[c][i];
        }
        let idx = match t[i] {
            'J' => 0,
            'O' => 1,
            'I' => 2,
            _ => unreachable!(),
        };
        pref[idx][i + 1] += 1; // 各文字の累積個数
    }

    let mismatch = |c: usize, l: usize, r: usize| -> usize {
        let cnt = pref[c][r] - pref[c][l];
        (r - l) - cnt // 区間 [l, r) を全部 c に揃える変更回数
    };

    let mut dp = vec![0usize; 2 * n + 1];
    let mut len = 1usize; // 現在の level-1 の長さ = 4^(level-1)

    for _level in 1..=k {
        let mut ndp = vec![0usize; 2 * n + 1];
        for start in 0..=2 * n - 4 * len {
            ndp[start] = mismatch(0, start, start + len) +                 // J を len 個
                mismatch(1, start + len, start + 2 * len) +       // O を len 個
                mismatch(2, start + 2 * len, start + 3 * len) +   // I を len 個
                dp[start + 3 * len]; // 最後は level-1 JOI 列
        }
        dp = ndp;
        len *= 4; // 長さを次の level に進める
    }

    let ans = (0..n).map(|start| dp[start]).min().unwrap(); // どの切り方でも最小を取る
    println!("{}", ans);
}
