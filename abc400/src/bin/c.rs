use proconio::input;

/// https://atcoder.jp/contests/abc400/tasks/abc400_c
/// https://atcoder.jp/contests/abc400/editorial/12634
///
/// a = 1,...,60, b = 奇数 とすると、
/// aの全探索で解ける
///
/// b^2 の個数を求めるのに、(N / 2^a)^(1/2)以下の奇数の個数を求めると言い換えれば良かった
/// なぜかb側も数字を全列挙しようと考えて、10^18というところで思考停止していた、、
/// 奇数の個数がそのまま答えになりうることを気づけなかった
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let mut a_pow = 1;
    for _ in 1..=60 {
        a_pow *= 2;
        if a_pow > n {
            continue;
        }
        // let b = ((n / a_pow) as f64).sqrt() as usize; // 誤差が生まれる
        let b = u64_floor_sqrt(n as u64 / a_pow as u64);
        ans += (b + 1) / 2;
    }

    println!("{}", ans);
}

/// f64の仮数部は53bitなので、nが2^53を超えると誤差が生まれる
/// そのため、u64の平方根を求める関数を自前で実装する必要がある
/// https://rsk0315.hatenablog.com/entry/2023/11/07/221428
pub fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}
