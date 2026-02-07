use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    let mut ans: usize = n;
    let mut left = 0;
    *map.entry(a[left]).or_default() += 1;
    for right in 1..n {
        let mut ok = true;
        let range_l = (a[right] + 1).saturating_sub(d);
        if let Some((&next, _)) = map.range(range_l..).next() {
            if next < a[right] + d {
                ok = false;
            }
        }

        while !ok && left < right {
            *map.entry(a[left]).or_default() -= 1;
            if let Some(&v) = map.get(&a[left]) {
                if v == 0 {
                    map.remove(&a[left]);
                }
            }
            left += 1;

            let mut ok_2 = true;
            let range_l = (a[right] + 1).saturating_sub(d);
            if let Some((&next, _)) = map.range(range_l..).next() {
                if next < a[right] + d {
                    ok_2 = false;
                }
            }

            if ok_2 {
                break;
            }
        }

        *map.entry(a[right]).or_default() += 1;
        ans += right - left;
    }

    println!("{}", ans);
}
