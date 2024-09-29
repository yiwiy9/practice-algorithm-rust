use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 12],
    }

    let mut ans = 0;
    for (i, s_i) in s.iter().enumerate() {
        if s_i.len() == i + 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
