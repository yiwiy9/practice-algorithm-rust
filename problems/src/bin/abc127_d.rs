use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        bc: [(usize,usize); m],
    }

    let mut cnt_map = BTreeMap::new();
    for &a_i in &a {
        cnt_map.entry(a_i).and_modify(|cur| *cur += 1).or_insert(1);
    }
    for &(b, c) in &bc {
        cnt_map.entry(c).and_modify(|cur| *cur += b).or_insert(b);
    }

    let mut used_cnt = 0;
    let mut ans = 0;
    for (num, cnt) in cnt_map.iter().rev() {
        if used_cnt >= n {
            break;
        } else if used_cnt + cnt >= n {
            ans += num * (n - used_cnt);
            used_cnt += n - used_cnt;
        } else {
            ans += num * cnt;
            used_cnt += cnt;
        }
    }

    println!("{}", ans);
}
