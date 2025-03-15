use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let s_max_len = s.iter().map(|s_i| s_i.len()).max().unwrap();
    let mut t = vec![vec!['*'; n]; s_max_len];
    for i in 0..n {
        for j in 0..s[i].len() {
            t[j][i] = s[i][j];
        }
    }

    for t_j in &t {
        let mut t_trimmed = vec![];

        let mut is_trimmed = true;
        for &t_j_i in t_j {
            if is_trimmed && t_j_i == '*' {
                continue;
            }
            is_trimmed = false;
            t_trimmed.push(t_j_i);
        }

        println!("{}", t_trimmed.iter().rev().join(""));
    }
}
