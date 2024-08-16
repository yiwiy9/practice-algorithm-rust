use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(u8, usize); q],
    }

    let mut cut_woods = BTreeSet::new();
    for (c, x) in cx {
        match c {
            1 => {
                cut_woods.insert(x);
            }
            2 => {
                let left_edge = cut_woods.range(..x).next_back().copied().unwrap_or(0);
                let right_edge = cut_woods.range(x..).next().copied().unwrap_or(l);
                println!("{}", right_edge - left_edge);
            }
            _ => unreachable!(),
        }
    }
}
