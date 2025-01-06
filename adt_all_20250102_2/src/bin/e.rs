use proconio::{input, marker::Chars};
use std::collections::{BTreeMap, HashMap};

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: Chars,
    }

    let mut ok = false;
    let mut map = HashMap::new();
    for i in 0..n {
        match s[i] {
            'L' => {
                let (x, y) = xy[i];
                let set = map.entry(y).or_insert(BTreeMap::new());

                if let Some((_, &prev_dir)) = set.range(..x).next_back() {
                    if prev_dir == 'R' {
                        ok = true;
                        break;
                    }
                }

                set.insert(x, 'L');
            }
            'R' => {
                let (x, y) = xy[i];
                let set = map.entry(y).or_insert(BTreeMap::new());

                if let Some((_, &next_dir)) = set.range(x..).next() {
                    if next_dir == 'L' {
                        ok = true;
                        break;
                    }
                }

                set.insert(x, 'R');
            }
            _ => unreachable!(),
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
