use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut equal_cnt = 0;
    for (x, &t_x) in t.iter().enumerate() {
        let s_x = s[s.len() - t.len() + x];
        if s_x == t_x || s_x == '?' || t_x == '?' {
            equal_cnt += 1;
        }
    }

    println!("{}", if equal_cnt == t.len() { "Yes" } else { "No" });

    for (x, &t_x) in t.iter().enumerate() {
        let before_s_x = s[s.len() - t.len() + x];
        if before_s_x == t_x || before_s_x == '?' || t_x == '?' {
            equal_cnt -= 1;
        }
        if s[x] == t_x || s[x] == '?' || t_x == '?' {
            equal_cnt += 1;
        }
        println!("{}", if equal_cnt == t.len() { "Yes" } else { "No" });
    }
}
