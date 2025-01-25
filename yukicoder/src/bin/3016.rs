use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n-1],
    }

    a.sort();
    b.sort();

    let mut cur = (0..n - 1).map(|i| (a[i] - b[i]).abs()).sum::<i64>();

    let mut map = std::collections::BTreeMap::new();
    map.insert(cur, BTreeSet::from([a[n - 1]]));

    for i in (0..n - 1).rev() {
        cur -= (a[i] - b[i]).abs();
        cur += (a[i + 1] - b[i]).abs();
        map.entry(cur).or_insert(BTreeSet::new()).insert(a[i]);
    }

    let min_set = map.iter().next().unwrap().1;

    println!("{}", min_set.len());
    println!(
        "{}",
        min_set
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
