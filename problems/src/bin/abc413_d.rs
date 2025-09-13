use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut a: [i64; n],
        }

        let mut cnt_map: HashMap<i64, i64> = HashMap::new();
        for &x in &a {
            *cnt_map.entry(x).or_insert(0) += 1;
        }

        if cnt_map.len() == 2 {
            let keys = cnt_map.keys().cloned().collect::<Vec<_>>();
            if keys[0] == -keys[1] && (cnt_map[&keys[0]] - cnt_map[&keys[1]]).abs() <= 1 {
                println!("Yes");
                continue;
            }
        }

        a.sort_by_key(|a| a.abs());

        let mut ok = true;
        for i in 2..n {
            if a[i] * a[0] != a[i - 1] * a[1] {
                ok = false;
                break;
            }
        }

        println!("{}", if ok { "Yes" } else { "No" });
    }
}
