use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc376/editorial/11192
/// - from, to, ng を考える
/// - from, to は入れ替えても同じ（このように、固定できる大小関係を固定しておくことで場合分けを減らすテクニックはしばしば有効です。）
/// - 場合分けは 2 つだけ
///    1. from < ng < to
///    2. それ以外
fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }

    let mut l_pos = 0;
    let mut r_pos = 1;

    let mut ans = 0;
    for (h, t) in ht {
        if h == 'L' {
            if l_pos < r_pos {
                if l_pos <= t && t < r_pos {
                    ans += t - l_pos;
                    l_pos = t;
                } else if t < l_pos {
                    ans += l_pos - t;
                    l_pos = t;
                } else {
                    ans += l_pos + (n - t);
                    l_pos = t;
                }
            } else {
                if r_pos < t && t <= l_pos {
                    ans += l_pos - t;
                    l_pos = t;
                } else if l_pos < t {
                    ans += t - l_pos;
                    l_pos = t;
                } else {
                    ans += t + (n - l_pos);
                    l_pos = t;
                }
            }
        } else {
            if l_pos < r_pos {
                if l_pos < t && t <= r_pos {
                    ans += r_pos - t;
                    r_pos = t;
                } else if r_pos < t {
                    ans += t - r_pos;
                    r_pos = t;
                } else {
                    ans += t + (n - r_pos);
                    r_pos = t;
                }
            } else {
                if r_pos <= t && t < l_pos {
                    ans += t - r_pos;
                    r_pos = t;
                } else if t < r_pos {
                    ans += r_pos - t;
                    r_pos = t;
                } else {
                    ans += r_pos + (n - t);
                    r_pos = t;
                }
            }
        }
    }

    println!("{}", ans);
}
