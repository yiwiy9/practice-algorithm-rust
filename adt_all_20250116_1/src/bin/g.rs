use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/adt_all_20250116_1/tasks/abc242_d
/// https://atcoder.jp/contests/adt_all_20250116_1/editorial/3520
///
/// k=0, t=0 の場合をまず考える
/// その後、0 < t, k の場合を観察すると、周期性が見え
/// => t-1を用いて表現することができる。２分木であることを考えると、k/2に着目することに気付ける
fn main() {
    input! {
        s: Chars,
        q: usize,
    }

    for _ in 0..q {
        input! {
            t: usize,
            k: usize,
        }
        println!("{}", rec(&s, t, k - 1));
    }
}

fn rec(s: &Vec<char>, t: usize, k: usize) -> char {
    if t == 0 {
        return s[k];
    }
    if k == 0 {
        return ('A' as u8 + ((s[0] as usize - 'A' as usize + t) % 3) as u8) as char;
    }

    return ('A' as u8 + ((rec(s, t - 1, k / 2) as usize - 'A' as usize + k % 2 + 1) % 3) as u8)
        as char;
}
