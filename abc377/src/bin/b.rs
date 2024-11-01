use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }

    let mut row_set = std::collections::HashSet::new();
    let mut col_set = std::collections::HashSet::new();
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                row_set.insert(i);
                col_set.insert(j);
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if row_set.contains(&i) || col_set.contains(&j) {
                continue;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
