use proconio::{input, marker::Chars};

const JOKER: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_cnt = vec![0; 26];
    let mut s_at_cnt = 0;
    for &c in &s {
        if c == '@' {
            s_at_cnt += 1;
        } else {
            s_cnt[c as usize - 'a' as usize] += 1;
        }
    }

    let mut t_cnt = vec![0; 26];
    let mut t_at_cnt = 0;
    for &c in &t {
        if c == '@' {
            t_at_cnt += 1;
        } else {
            t_cnt[c as usize - 'a' as usize] += 1;
        }
    }

    let mut ok = true;
    for i in 0..26 {
        if s_cnt[i] == t_cnt[i] {
            continue;
        } else if s_cnt[i] < t_cnt[i] {
            if s_at_cnt + s_cnt[i] >= t_cnt[i] && JOKER.contains(&(('a' as u8 + i as u8) as char)) {
                s_at_cnt -= t_cnt[i] - s_cnt[i];
            } else {
                ok = false;
                break;
            }
        } else {
            if t_at_cnt + t_cnt[i] >= s_cnt[i] && JOKER.contains(&(('a' as u8 + i as u8) as char)) {
                t_at_cnt -= s_cnt[i] - t_cnt[i];
            } else {
                ok = false;
                break;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
