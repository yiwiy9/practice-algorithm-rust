use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut map = BTreeMap::new();
    let mut cur = a[0];
    let mut before = a[0];
    let mut start_num = a[0];
    let mut i = 1;
    while i < n {
        if a[i] == before || a[i] == before + 1 {
            cur += a[i];
        } else {
            map.insert(start_num, cur);

            cur = a[i];
            start_num = a[i];
        }
        before = a[i];
        i += 1;
    }

    map.insert(
        start_num,
        cur + if let Some(&v) = map.get(&0) { v } else { 0 },
    );

    let max_val = map.values().max().unwrap_or(&0);
    let sum_val = a.iter().sum::<usize>();

    println!("{}", sum_val - max_val);
}
