use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_usize;
    for i in 0..s.len() - 1 {
        ans += 1;

        let cur_s_num = s[i] as usize - '0' as usize;
        let next_s_num = s[i + 1] as usize - '0' as usize;
        if cur_s_num >= next_s_num {
            let diff = cur_s_num - next_s_num;
            ans += diff;
        } else {
            let diff = next_s_num - cur_s_num;
            ans += 10 - diff;
        }
    }

    ans += 1;
    let last_s_num = s[s.len() - 1] as usize - '0' as usize;
    ans += last_s_num;

    println!("{}", ans);
}
