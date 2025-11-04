use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut min_h = h;
    let mut max_h = 0;
    let mut min_w = w;
    let mut max_w = 0;
    for (i, row) in s.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                min_h = min_h.min(i);
                max_h = max_h.max(i);
                min_w = min_w.min(j);
                max_w = max_w.max(j);
            }
        }
    }

    let mut ok = true;
    for (i, row) in s.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if (min_h..=max_h).contains(&i) && (min_w..=max_w).contains(&j) && c == '.' {
                ok = false;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
