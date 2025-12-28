use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let mut ans = 1 << 30;
    for i in 0..=n - m {
        let mut cur = 0;
        for j in 0..m {
            let s_j = s[i + j].to_digit(10).unwrap();
            let t_j = t[j].to_digit(10).unwrap();
            cur += if s_j >= t_j {
                s_j - t_j
            } else {
                s_j + 10 - t_j
            };
        }
        ans = ans.min(cur);
    }

    println!("{}", ans);
}
