use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..=9 {
        if s[i] == 'x' {
            continue;
        }
        for j in 0..=9 {
            if s[j] == 'x' {
                continue;
            }
            for k in 0..=9 {
                if s[k] == 'x' {
                    continue;
                }
                for l in 0..=9 {
                    if s[l] == 'x' {
                        continue;
                    }
                    let mut ok = true;
                    for (s_i, &c) in s.iter().enumerate() {
                        if c == 'o' && s_i != i && s_i != j && s_i != k && s_i != l {
                            ok = false;
                        }
                    }
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
