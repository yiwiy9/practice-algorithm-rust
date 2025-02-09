use num::Float;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }

    let mut k = vec![0; n];
    let mut a = vec![HashMap::new(); n];
    for i in 0..n {
        input! {
            k_i: usize,
            a_i: [usize; k_i],
        }
        k[i] = k_i;
        for &a_ij in &a_i {
            a[i].entry(a_ij).and_modify(|cur| *cur += 1).or_insert(1);
        }
    }

    let mut ans = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let mut count = 0_usize;
            for (&a_ij, &cnt_ij) in &a[i] {
                if a[j].contains_key(&a_ij) {
                    count += cnt_ij * a[j][&a_ij];
                }
            }

            let p = count as f64 / k[i] as f64 / k[j] as f64;

            ans = ans.max(p);
        }
    }

    println!("{}", ans);
}
