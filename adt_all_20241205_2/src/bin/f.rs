use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize,usize); q],
    }

    let mut a_cnt_map = HashMap::new();
    let mut a_cnt_idx_map = HashMap::new();
    for i in 0..n {
        let cnt = a_cnt_map.entry(a[i]).or_insert(0);
        *cnt += 1;
        a_cnt_idx_map.insert((a[i], *cnt), (i + 1) as i32);
    }

    for &(x, k) in &xk {
        println!("{}", a_cnt_idx_map.get(&(x, k)).unwrap_or(&(-1)));
    }
}
