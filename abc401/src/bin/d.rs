use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: i64,
        s: Chars,
    }

    let mut t = vec!['?'; n];
    for i in 0..n {
        if s[i] == 'o' {
            if i > 0 {
                t[i - 1] = '.';
            }
            if i < n - 1 {
                t[i + 1] = '.';
            }
        }
        if t[i] != '.' {
            t[i] = s[i];
        }
    }

    let mut remain_k = k - t.iter().filter(|&&x| x == 'o').count() as i64;
    if remain_k == 0 {
        println!(
            "{}",
            t.iter().map(|&c| if c == '?' { '.' } else { c }).join("")
        );
        return;
    }

    let mut remains = vec![];
    let mut cur = 0;
    for i in 0..n {
        if t[i] == '?' {
            cur += 1;
        } else {
            if cur > 0 {
                remains.push((i as i64 - cur, cur));
            }
            cur = 0;
        }
    }
    if cur > 0 {
        remains.push((n as i64 - cur, cur));
    }

    let mut odd_remains = vec![];
    for &(start_i, cnt) in &remains {
        if cnt % 2 == 0 {
            remain_k -= cnt / 2;
            continue;
        }
        remain_k -= (cnt + 1) / 2;
        odd_remains.push((start_i, cnt));
    }

    if remain_k >= 0 {
        for &(start_i, cnt) in &odd_remains {
            let mut cur = 0;
            for i in start_i..start_i + cnt {
                if cur % 2 == 0 {
                    t[i as usize] = 'o';
                } else {
                    t[i as usize] = '.';
                }
                cur += 1;
            }
        }
    }

    println!("{}", t.iter().join(""));
}
