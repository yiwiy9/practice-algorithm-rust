use proconio::{input, marker::Chars};
use std::collections::HashMap;

// ハッシュ化するための定数
const B: usize = 200;
const MOD: usize = 10_000_000_000_000_000 + 61;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    s.sort();

    let mut map = HashMap::new();
    let mut ans = 0_usize;
    for s_i in &s {
        // 文字列のままやるとTLEになる（t.clone()）
        // let mut t = vec![];
        // for &c in s_i {
        //     t.push(c);
        //     if let Some(&j) = map.get(&t) {
        //         ans += j;
        //     }
        //     *map.entry(t.clone()).or_insert(0) += 1;
        // }

        // 文字列をhash化する
        // https://atcoder.jp/contests/abc353/editorial/9980
        let mut hash = 0;
        for &c in s_i {
            hash = (hash * B + c as usize) % MOD;
            if let Some(&j) = map.get(&hash) {
                ans += j;
            }
            *map.entry(hash).or_insert(0) += 1;
        }
    }

    println!("{}", ans);
}
