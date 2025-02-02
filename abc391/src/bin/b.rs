use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    for a in 0..n {
        for b in 0..n {
            let mut ok = true;
            for i in 0..m {
                for j in 0..m {
                    if a + i >= n || b + j >= n || s[a + i][b + j] != t[i][j] {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                println!("{} {}", a + 1, b + 1);
                return;
            }
        }
    }
}
