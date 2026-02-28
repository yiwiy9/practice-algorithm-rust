use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans: usize = 0;
    let mut j = 0;
    for &t_i in &t {
        if j < s.len() && t_i == s[j] {
            j += 1;
            continue;
        }

        if t_i == 'A' {
            ans += 1;
            continue;
        }

        while j < s.len() && s[j] == 'A' {
            ans += 1;
            j += 1;
        }

        if j < s.len() && t_i == s[j] {
            j += 1;
            continue;
        }

        println!("-1");
        return;
    }

    for jj in j..s.len() {
        if s[jj] != 'A' {
            println!("-1");
            return;
        }
        ans += 1;
    }

    println!("{}", ans);
}
