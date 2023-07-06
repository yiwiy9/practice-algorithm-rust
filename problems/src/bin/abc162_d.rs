use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut r_cnt: i64 = 0;
    let mut g_cnt: i64 = 0;
    let mut b_cnt: i64 = 0;
    for c in &s {
        match *c {
            'R' => r_cnt += 1,
            'G' => g_cnt += 1,
            'B' => b_cnt += 1,
            _ => unreachable!(),
        }
    }

    let mut ans = r_cnt * g_cnt * b_cnt;
    for i in 0..n {
        for j in i + 1..n {
            if j + j - i >= n {
                continue;
            }
            if s[i] != s[j] && s[j] != s[j + j - i] && s[i] != s[j + j - i] {
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
