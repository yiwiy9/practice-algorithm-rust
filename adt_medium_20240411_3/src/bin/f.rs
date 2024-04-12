use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_char_cnt = vec![0; 26];
    let mut s_at_cnt = 0;
    let mut t_char_cnt = vec![0; 26];
    let mut t_at_cnt = 0;

    for c in s {
        if c == '@' {
            s_at_cnt += 1;
        } else {
            s_char_cnt[(c as u8 - b'a') as usize] += 1;
        }
    }
    for c in t {
        if c == '@' {
            t_at_cnt += 1;
        } else {
            t_char_cnt[(c as u8 - b'a') as usize] += 1;
        }
    }

    let atcoder = "atcoder"
        .chars()
        .map(|c| (c as u8 - b'a') as usize)
        .collect::<Vec<_>>();

    let mut ok = true;
    for i in 0..26 {
        if s_char_cnt[i] == t_char_cnt[i] {
            continue;
        }

        if !atcoder.contains(&i) {
            ok = false;
            break;
        }

        if s_char_cnt[i] > t_char_cnt[i] {
            let diff = s_char_cnt[i] - t_char_cnt[i];
            if t_at_cnt < diff {
                ok = false;
                break;
            }
            t_at_cnt -= diff;
        } else {
            let diff = t_char_cnt[i] - s_char_cnt[i];
            if s_at_cnt < diff {
                ok = false;
                break;
            }
            s_at_cnt -= diff;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
