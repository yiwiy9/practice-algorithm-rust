use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut min_h = h;
    let mut min_w = w;
    let mut max_h = 0;
    let mut max_w = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                min_h = std::cmp::min(min_h, i);
                min_w = std::cmp::min(min_w, j);
                max_h = std::cmp::max(max_h, i);
                max_w = std::cmp::max(max_w, j);
            }
        }
    }

    let mut ok = true;
    for i in min_h..=max_h {
        for j in min_w..=max_w {
            if s[i][j] == '.' {
                ok = false;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
