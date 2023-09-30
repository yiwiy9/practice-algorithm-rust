use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut cnt = 0;
    for (i, &c) in s.iter().enumerate() {
        match c {
            'A' => {
                ans = i + 1;
                cnt = 1;
            }
            'B' => {
                if cnt == 1 {
                    cnt += 1;
                } else {
                    ans = 0;
                    cnt = 0;
                }
            }
            'C' => {
                if cnt == 2 {
                    println!("{}", ans);
                    return;
                } else {
                    ans = 0;
                    cnt = 0;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", -1);
}
