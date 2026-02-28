use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: usize = 0;
    let mut cnt_vec = [0; 2];
    for &c in &s {
        match c {
            'A' => {
                cnt_vec[0] += 1;
            }
            'B' => {
                if cnt_vec[0] > 0 {
                    cnt_vec[0] -= 1;
                    cnt_vec[1] += 1;
                }
            }
            'C' => {
                if cnt_vec[1] > 0 {
                    cnt_vec[1] -= 1;
                    ans += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
