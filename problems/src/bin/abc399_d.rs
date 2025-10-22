use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        t: usize,
    }

    // 1 2 1 ←このケースを数え上げないようにする必要がある
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; 2*n],
        }

        let mut set = HashSet::new();
        let mut ans = 0;
        let mut i = 0;
        while i < 2 * n - 1 {
            if i > 0 && a[i - 1] == a[i] {
                i += 1;
                continue;
            }

            if a[i] == a[i + 1] {
                i += 1;
                continue;
            }

            let pair = (a[i].min(a[i + 1]), a[i].max(a[i + 1]));
            if !set.contains(&pair) {
                set.insert(pair);
                if i + 2 < 2 * n && a[i] == a[i + 2] {
                    i += 1;
                }
                i += 1;
                continue;
            }

            ans += 1;
            i += 1;
        }

        println!("{}", ans);
    }
}
