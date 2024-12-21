use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut cur_m = m;
    let mut cur_l = 0;
    for &c in &s {
        match c {
            '0' => {
                cur_m = m;
                cur_l = ans;
            }
            '1' => {
                if cur_m > 0 {
                    cur_m -= 1;
                } else if cur_l > 0 {
                    cur_l -= 1;
                } else {
                    ans += 1;
                }
            }
            '2' => {
                if cur_l > 0 {
                    cur_l -= 1;
                } else {
                    ans += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
