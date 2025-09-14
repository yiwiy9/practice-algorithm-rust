use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); n],
        cd: [(Usize1,Usize1); m],
    }

    let mut rows = vec![BTreeMap::new(); h];
    let mut columns = vec![BTreeMap::new(); w];
    for &(a, b) in &ab {
        rows[a].insert(b, true);
        columns[b].insert(a, true);
    }
    for &(c, d) in &cd {
        rows[c].insert(d, false);
        columns[d].insert(c, false);
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut ok = false;

            if rows[i].get(&j) == Some(&false) {
                continue;
            }

            if let Some((_, &is_light)) = rows[i].range(j..).next() {
                if is_light {
                    ok = true;
                }
            }
            if let Some((_, &is_light)) = rows[i].range(..j).next_back() {
                if is_light {
                    ok = true;
                }
            }

            if let Some((_, &is_light)) = columns[j].range(i..).next() {
                if is_light {
                    ok = true;
                }
            }
            if let Some((_, &is_light)) = columns[j].range(..i).next_back() {
                if is_light {
                    ok = true;
                }
            }

            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
