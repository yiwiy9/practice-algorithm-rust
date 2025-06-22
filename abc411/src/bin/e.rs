use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/abc411/tasks/abc411_e
/// https://atcoder.jp/contests/abc411/editorial/13361
///
/// 競技プログラミングにおける確率・期待値の問題では、
/// 「何かの最大値が特定の値に一致する確率を求めるより、最大値が特定の値以下になる確率を求める方が往々にして簡単である」
/// ということを利用して式変形を行うことがしばしばあります。
fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut s = a.iter().flat_map(|x| x.iter().copied()).collect::<Vec<_>>();
    s.sort();
    s.dedup();

    let k = s.len();
    let mut upd = vec![vec![]; k];
    for i in 0..n {
        for j in 0..6 {
            let idx = s.lower_bound(&a[i][j]);
            upd[idx].push(i);
        }
    }

    let mut ans = Mint::new(0);
    let mut b = vec![0; n];
    let mut p = Mint::new(1);
    let mut zero_cnt = n;
    for i in 0..k - 1 {
        for &j in &upd[i] {
            if b[j] == 0 {
                zero_cnt -= 1;
            } else {
                p /= Mint::new(b[j]);
            }
            b[j] += 1;
            p *= Mint::new(b[j]);
        }
        if zero_cnt == 0 {
            ans -= p * Mint::new(s[i + 1] - s[i]);
        }
    }
    ans /= Mint::new(6).pow(n as u64);

    ans += Mint::new(s[k - 1]);

    println!("{}", ans.val());
}
