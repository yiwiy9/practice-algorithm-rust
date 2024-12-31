use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyc: [(usize,usize,char); m],
    }

    let mut black = vec![];
    let mut white = vec![];
    for (x, y, c) in xyc {
        if c == 'B' {
            black.push((x, y));
        } else {
            white.push((x, y));
        }
    }

    black.sort_by(|a, b| b.1.cmp(&a.1));
    let mut black_row_map = BTreeMap::new();
    for (x, y) in black {
        if let Some(set) = black_row_map.range(x..).next() {
            continue;
        }
        black_row_map.insert(x, y);
    }

    let mut is_ok = true;
    for (x, y) in white {
        if let Some((_, &black_y)) = black_row_map.range(x..).next() {
            if black_y >= y {
                is_ok = false;
                break;
            }
        }
    }

    println!("{}", if is_ok { "Yes" } else { "No" });
}
