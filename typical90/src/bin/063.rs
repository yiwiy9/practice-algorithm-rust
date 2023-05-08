use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut ans = 0;
    for bit in 1..(1 << h) {
        let mut map = BTreeMap::new();
        for j in 0..w {
            let mut val = 0;
            let mut cnt = 0;
            let mut is_same = true;
            for (i, row) in p.iter().enumerate() {
                if bit & (1 << i) != 0 {
                    if val != 0 {
                        is_same &= row[j] == val;
                    }
                    val = row[j];
                    cnt += 1;
                }
            }
            if is_same {
                map.entry(val).and_modify(|cur| *cur += cnt).or_insert(cnt);
            }
        }
        ans = ans.max(*map.values().max().unwrap_or(&0));
    }

    println!("{}", ans);
}
