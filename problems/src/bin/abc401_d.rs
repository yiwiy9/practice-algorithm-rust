use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut remain_k = k;
    let mut t = s.clone();
    for (i, &c) in s.iter().enumerate() {
        if c == 'o' {
            if i > 0 {
                t[i - 1] = '.';
            }
            if i < n - 1 {
                t[i + 1] = '.';
            }
            remain_k -= 1;
        }
    }

    if remain_k == 0 {
        println!(
            "{}",
            t.iter().map(|&c| if c == '?' { '.' } else { c }).join("")
        );
        return;
    }

    let mut room_cnt = 0;
    let mut odd_cnt = 0;
    let mut cur_cnt = 0;
    for &c in &t {
        if c == '?' {
            cur_cnt += 1;
            continue;
        }

        room_cnt += cur_cnt / 2;
        odd_cnt += if cur_cnt % 2 == 1 { 1 } else { 0 };
        cur_cnt = 0;
    }
    room_cnt += cur_cnt / 2;
    odd_cnt += if cur_cnt % 2 == 1 { 1 } else { 0 };

    if room_cnt + odd_cnt == remain_k {
        let t_clone = t.clone();
        let mut cur_cnt = 0;
        for (i, &c) in t_clone.iter().enumerate() {
            if c == '?' {
                cur_cnt += 1;
                continue;
            }

            if cur_cnt % 2 == 1 {
                for j in 1..=cur_cnt {
                    t[i - j] = if j % 2 == 1 { 'o' } else { '.' };
                }
            }
            cur_cnt = 0;
        }
        if cur_cnt % 2 == 1 {
            for j in 1..=cur_cnt {
                t[n - j] = if j % 2 == 1 { 'o' } else { '.' };
            }
        }
    }

    println!("{}", t.iter().join(""));
}
