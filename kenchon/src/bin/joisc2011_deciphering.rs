use ac_library::ModInt as Mint;
use proconio::{input, marker::Chars};
use std::collections::HashSet;

const MOD: u32 = 10000000;

/// https://atcoder.jp/contests/joisc2011/tasks/joisc2011_deciphering
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   前から i 文字まで見たときに、作れる異なる合法部分列のうち末尾が j であるものの個数だけ持つ
/// - それについて、何が分かれば答えになる？
///   各末尾 j ごとの個数が分かればよく、最後に j=1..26 を合計すれば答え
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   次に s[i] を付けられるかは直前文字 j と s[i] だけで決まる
///   また、同じ文字列の重複は末尾が同じ集合の中でだけ起こるので、列全体は持たなくてよい
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   各 s[i] について、禁止 pair でない全末尾 j から next[s[i]] に dp[j] を足す
///   このとき以前からあった末尾 s[i] の集合をちょうど再生成するので、dp[s[i]] を 1 回引いて重複を消す
///
/// AC: 37分
fn main() {
    input! {
        _l: usize,
        s: Chars,
        m: usize,
        ab: [(char, char); m],
    }

    let mut ab_set = HashSet::new();
    for &(a, b) in &ab {
        let a_num = a as usize - 'A' as usize + 1;
        let b_num = b as usize - 'A' as usize + 1;
        ab_set.insert((a_num, b_num));
    }

    Mint::set_modulus(MOD);
    let mut dp = vec![Mint::new(0); 27];
    dp[0] = Mint::new(1);
    for &s_i in &s {
        let s_i_num = s_i as usize - 'A' as usize + 1;
        let mut next_dp = dp.clone();
        for j in 0..27 {
            if ab_set.contains(&(j, s_i_num)) {
                continue;
            }
            next_dp[s_i_num] += dp[j];
        }
        next_dp[s_i_num] -= dp[s_i_num];
        dp = next_dp;
    }

    let mut ans = Mint::new(0);
    for j in 1..27 {
        ans += dp[j];
    }

    println!("{}", ans);
}
