use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc158/tasks/abc158_e
/// https://drken1215.hatenablog.com/entry/2020/03/08/020200
/// S = 1533032 であれば、
/// S0 = 0
/// S1 = 2
/// S2 = 32
/// S3 = 32 ("032")
/// S4 = 3032
///
/// このとき、S の区間 [l,r) が条件を満たす条件は
/// (Sl − Sr) / 10^r ≡ 0 (mod.P)
///
/// P ≠ 2 or 5 の場合は、互いに素であるので
/// 両辺に 10^r かけて
/// Sl ≡ Sr (mod.P)
fn main() {
    input! {
        n: usize,
        p: usize,
        s: Chars,
    }

    if p == 2 || p == 5 {
        // 末尾がpで割り切れる場合の数
        println!(
            "{}",
            (0..n).fold(0, |acc, i| {
                acc + if (s[n - i - 1] as usize - '0' as usize) % p == 0 {
                    n - i
                } else {
                    0
                }
            }),
        );
        return;
    }

    let mut val = vec![0_i64; p];
    let mut cur = 0;
    val[cur] += 1;
    let mut mul = 1;
    for i in 0..n {
        cur = (cur + (s[n - i - 1] as usize - '0' as usize) * mul) % p;
        val[cur] += 1;
        mul = (mul * 10) % p;
    }

    println!(
        "{}",
        (0..p).fold(0, |acc, j| acc + val[j] * (val[j] - 1) / 2)
    );
}
