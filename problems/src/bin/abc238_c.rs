use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

/// ModIntの使い方
/// https://atcoder.github.io/ac-library/production/document_ja/modint.html
fn main() {
    input! {
        n: usize,
    }

    let mut ans = Mint::new(0);
    let mut cur = 1_usize;
    loop {
        if cur.saturating_mul(10) > n {
            let cnt = Mint::new(n - (cur - 1));
            let sum = (cnt * (cnt + 1)) / 2; // mod_invなしで計算できる
            ans += sum;
            break;
        }

        let cnt = Mint::new((cur * 10 - 1) - (cur - 1));
        let sum = (cnt * (cnt + 1)) / 2;
        ans += sum;
        cur *= 10;
    }

    println!("{}", ans.val());
}
