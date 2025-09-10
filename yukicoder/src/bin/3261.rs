use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    let mut used_y = HashSet::new();
    let mut used_9 = HashSet::new();
    for i in 0..h {
        for j in 1..w - 1 {
            if s[i][j] == 'y' && !used_y.contains(&(i, j)) {
                if s[i][j - 1] == '9' && !used_9.contains(&(i, j - 1)) {
                    s[i][j] = 'Y';
                    used_y.insert((i, j));
                    used_y.insert((i, j + 4));
                    used_9.insert((i, j - 1));
                }
                if s[i][j + 1] == '9' && !used_9.contains(&(i, j + 1)) {
                    s[i][j] = 'Y';
                    used_y.insert((i, j));
                    used_9.insert((i, j + 1));
                }
            }
        }
    }

    println!(
        "{}",
        s.iter()
            .map(|s_i| s_i.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
