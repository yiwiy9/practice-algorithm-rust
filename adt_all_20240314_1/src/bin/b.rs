use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for row in &s {
        for &c in row {
            if c == '#' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
