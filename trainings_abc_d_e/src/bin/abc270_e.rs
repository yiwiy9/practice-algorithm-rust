use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();
    for &a_i in &a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut i = n;
    let mut minus = 0;
    let mut before_num = 0;
    for (num, cnt) in map {
        let sub = (num - before_num) * i;
        if sub <= k {
            k -= sub;
            i -= cnt;
            minus += num - before_num;
            before_num = num;
        } else {
            let sub_minus = k / i;
            k -= sub_minus * i;
            minus += sub_minus;
            break;
        }
    }

    let mut ans = vec![0; n];
    for (i, &a_i) in a.iter().enumerate() {
        if a_i > minus {
            ans[i] = a_i - minus;
            if k > 0 {
                ans[i] -= 1;
                k -= 1;
            }
        }
    }

    println!("{}", ans.iter().join(" "))
}
