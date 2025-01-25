use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_usize;
    let mut one_cnt = 0;
    for &c in &s {
        if c == '1' {
            one_cnt += 1;
        } else {
            let set_cnt = one_cnt / 2;
            ans += set_cnt;
            one_cnt = set_cnt * 2;
        }
    }

    println!("{}", ans);
}
