use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let s_num = s[0] as u8 - 'a' as u8;
    let t_num = t[0] as u8 - 'a' as u8;

    let k = (t_num + 26 - s_num) % 26;

    for (i, &c) in s.iter().enumerate() {
        let new_c = (c as u8 - 'a' as u8 + k) % 26;
        let t_c = t[i] as u8 - 'a' as u8;
        if new_c != t_c {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
