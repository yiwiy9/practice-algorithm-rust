use itertools::Itertools as _;
use proconio::input;

/// https://atcoder.jp/contests/abc386/tasks/abc386_e
/// https://atcoder.jp/contests/abc386/editorial/11697
///
/// 敗因: XOR の性質を使って高速化する方法ばかり考えてしまい、実験による計算量見積りができていなかった
///
/// nCk <= 10^6 という特殊な制約を丁寧に考察する
/// 安易に全探索ができるわけではない、k > n/2 の場合は nC(n/2) が最大値となり、10^6 を大幅に超える可能性がある
/// この場合、nCk = nC(n-k) であることを利用して、nCk = nC(n-k) <= 10^6 となるように場合分けをする
///
/// 再帰関数でnCkをどのように計算するかのイメージを持っておくと良い
/// def func(x: list[int], i: int):
///     if len(x) == K:
///         # 長さが K であるようなインデックスの列 x がここで列挙される
///         return
///     if i == N:
///         return
///     func(x, i + 1)
///     func(x + [i], i + 1)
///
/// func([], 0)

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    if k <= n - k {
        for comb in (0..n).combinations(k) {
            let mut xor = 0;
            for &i in &comb {
                xor ^= a[i];
            }
            ans = ans.max(xor);
        }
    } else {
        let all_xor = a.iter().fold(0, |acc, &a_i| acc ^ a_i);
        for comb in (0..n).combinations(n - k) {
            let mut xor = all_xor;
            for &i in &comb {
                xor ^= a[i];
            }
            ans = ans.max(xor);
        }
    }

    println!("{}", ans);
}
