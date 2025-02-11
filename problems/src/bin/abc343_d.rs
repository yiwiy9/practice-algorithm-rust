use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(Usize1, usize); t],
    }

    let mut map = HashMap::new();
    map.insert(0, n);
    let mut points = vec![0; n];
    for &(a, b) in &ab {
        if let Some(&p) = map.get(&points[a]) {
            if p == 1 {
                map.remove(&points[a]);
            } else {
                map.entry(points[a]).and_modify(|p| *p -= 1);
            }
        }

        points[a] += b;
        map.entry(points[a]).and_modify(|p| *p += 1).or_insert(1);

        println!("{}", map.len());
    }
}
