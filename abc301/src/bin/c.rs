use proconio::{input, marker::Chars};

const AT: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_cnt = vec![0; 26];
    let mut s_at_cnt = 0;
    let mut t_cnt = vec![0; 26];
    let mut t_at_cnt = 0;

    for i in 0..s.len() {
        if s[i] == '@' {
            s_at_cnt += 1;
        } else {
            s_cnt[s[i] as usize - 'a' as usize] += 1;
        }
        if t[i] == '@' {
            t_at_cnt += 1;
        } else {
            t_cnt[t[i] as usize - 'a' as usize] += 1;
        }
    }

    let mut ok = true;
    for i in 0..26 {
        if s_cnt[i] == t_cnt[i] {
            continue;
        }
        let c = std::char::from_u32((i + 'a' as usize) as u32).unwrap();
        if !AT.iter().any(|e| *e == c) {
            ok = false;
            break;
        }

        if s_cnt[i] > t_cnt[i] {
            t_at_cnt -= s_cnt[i] - t_cnt[i];
            if t_at_cnt < 0 {
                ok = false;
                break;
            }
        } else {
            s_at_cnt -= t_cnt[i] - s_cnt[i];
            if s_at_cnt < 0 {
                ok = false;
                break;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
