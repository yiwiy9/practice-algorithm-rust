use proconio::input;
use std::collections::BTreeMap;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut cnt_map = BTreeMap::new();
    cnt_map.insert(a[0], 1);

    let mut a_arranged = vec![0; n - 1];
    *cnt_map.entry(a[1]).or_insert(0) += 1;
    a_arranged[0] = a[1];
    for i in 2..n {
        *cnt_map.entry(a[i]).or_insert(0) += 1;
        a_arranged[i - 1] = a[i].max(a_arranged[i - 2]);
    }

    let mut b_arranged = vec![0; n - 1];
    *cnt_map.entry(b[1]).or_insert(0) += 1;
    b_arranged[0] = b[1];
    for i in 2..n {
        *cnt_map.entry(b[i]).or_insert(0) += 1;
        b_arranged[i - 1] = b[i].max(b_arranged[i - 2]);
    }

    let mut a_left_cnt = vec![0; n - 1];
    for &b_i in &b_arranged {
        let upper_i = a_arranged.upper_bound(&b_i);
        if upper_i < n - 1 {
            a_left_cnt[upper_i] += 1;
        }
        *cnt_map.entry(b_i).or_insert(0) += upper_i;
    }

    let mut sum = 0;
    for i in 0..n - 1 {
        sum += a_left_cnt[i];
        *cnt_map.entry(a_arranged[i]).or_insert(0) += sum;
    }

    let mut ans = 0;
    let mut max_cnt = 0;
    for (&num, &cnt) in &cnt_map {
        if cnt >= max_cnt {
            ans = num;
            max_cnt = cnt
        }
    }

    println!("{} {}", ans, max_cnt);
}
