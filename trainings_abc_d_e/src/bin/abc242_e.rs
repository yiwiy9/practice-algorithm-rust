use proconio::{input, marker::Chars};
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc242/tasks/abc242_e
 * https://atcoder.jp/contests/abc242/editorial/3516
 */
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let mut ans = 1;

        let half = (n + 1) / 2;
        let mut multiple = 1;
        for i in (0..half).rev() {
            let extra_cnt = (s[i] as usize) - b'A' as usize;
            ans += extra_cnt * multiple;
            ans %= MOD;
            multiple *= 26;
            multiple %= MOD;
        }

        // 元の文字列が全桁回文より辞書順で小さい場合は、1を引く
        let mut ok = true;
        for i in (0..half).rev() {
            if (s[i] as usize) == (s[n - 1 - i] as usize) {
                continue;
            }
            ok = (s[i] as usize) < (s[n - 1 - i] as usize);
            break;
        }

        if !ok {
            ans += MOD - 1;
            ans %= MOD;
        }

        println!("{}", ans);
    }
}
