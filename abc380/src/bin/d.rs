use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

/**
 * https://atcoder.jp/contests/abc380/tasks/abc380_d
 * https://atcoder.jp/contests/abc380/editorial/11362
 *
 * B ブロック目の文字を特定したいとする。
 * B を 2 進表記した際、最も上位にある 1 が 2k の位であるとき、 B ブロック目は B−2k ブロック目の大小文字を反転したものになる。
 * <= 最後の操作で後ろに足されるブロックが全て最も上位にある 1 が 2k の位であるから
 *
 * つまり、B を 2 進表記した際の各桁に含まれる 1 の数が反転回数になる。
 */
fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [Usize1; q],
    }

    let mut ans = vec![];
    for k_i in k {
        let block_cnt = k_i / s.len();
        let remains = k_i % s.len();
        let is_reverse = block_cnt.count_ones() % 2 == 1;
        if is_reverse {
            ans.push(if s[remains].is_ascii_lowercase() {
                s[remains].to_ascii_uppercase()
            } else {
                s[remains].to_ascii_lowercase()
            });
        } else {
            ans.push(s[remains]);
        }
    }

    println!("{}", ans.iter().join(" "));
}
