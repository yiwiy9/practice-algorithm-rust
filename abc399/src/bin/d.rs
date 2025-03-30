use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; 2*n],
        }

        let mut cant_use_set = HashSet::new();
        for i in 1..2 * n {
            if a[i] == a[i - 1] {
                cant_use_set.insert(a[i]);
            }
        }

        let mut ans = 0_usize;
        let mut next = vec![HashSet::new(); n + 1];
        let mut last_insert = false;
        for i in 0..2 * n - 1 {
            if cant_use_set.contains(&a[i]) {
                last_insert = false;
                continue;
            }

            if !cant_use_set.contains(&a[i + 1]) {
                if last_insert && a[i - 1] == a[i + 1] {
                    last_insert = false;
                    continue;
                }

                last_insert = true;

                let pair = if a[i] < a[i + 1] {
                    (a[i], a[i + 1])
                } else {
                    (a[i + 1], a[i])
                };

                if next[pair.0].contains(&pair.1) {
                    ans += 1;
                } else {
                    next[pair.0].insert(pair.1);
                }
                continue;
            }

            last_insert = false;
        }
        println!("{}", ans);
    }
}
