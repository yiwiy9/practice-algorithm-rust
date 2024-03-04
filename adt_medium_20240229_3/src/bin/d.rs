use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut tokens = vec![(0, 0); 2];
    let mut token_cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                tokens[token_cnt] = (i, j);
                token_cnt += 1;
            }
        }
    }

    println!(
        "{}",
        tokens[0].0.abs_diff(tokens[1].0) + tokens[0].1.abs_diff(tokens[1].1)
    );
}
