use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc310/tasks/abc310_e
/// https://atcoder.jp/contests/abc310/editorial/6784
///
/// 問題文通り、jごとの計算を O(1) でできないか？ という発想に囚われているのがゴミ
/// => 0 or 1 が決まればそれ以降の答えは同じだな、というところまでは気づいていた
/// => この情報を使って、問題を小問題に分割できる断面を見つけ出す必要がある
/// => iごとに問題を分割し、それまでの計算結果（0 or 1 の現時点の個数）を使って、外側のシグマでDPする
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut zero = 0;
    let mut one = 0;
    let mut ans = 0_usize;
    for &s_i in &s {
        let new_zero;
        let new_one;
        if s_i == '0' {
            new_zero = 1; // 自身の0
            new_one = zero + one; // 0の場合必ず1
        } else {
            new_zero = one;
            new_one = zero + 1;
        }
        ans += new_one;
        zero = new_zero;
        one = new_one;
    }

    println!("{}", ans);
}
