use proconio::input;
use std::collections::{BTreeSet, HashMap};

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            m: usize,
            pe: [(usize,usize); m],
        }

        for &(p, e) in &pe {
            if let Some((v_e, v_cnt)) = map.get_mut(&p) {
                match e.cmp(v_e) {
                    std::cmp::Ordering::Greater => {
                        *v_e = e;
                        *v_cnt = 1;
                    }
                    std::cmp::Ordering::Equal => {
                        *v_cnt += 1;
                    }
                    std::cmp::Ordering::Less => {}
                }
            } else {
                map.insert(p, (e, 1));
            }
        }

        a.push(pe);
    }

    let mut lcm_set = BTreeSet::new();
    for pe in &a {
        let mut cur_set = BTreeSet::new();
        for &(p, e) in pe {
            let (v_e, v_cnt) = *map.get(&p).unwrap();
            if e == v_e && v_cnt == 1 {
                cur_set.insert((p, e));
            }
        }
        lcm_set.insert(cur_set);
    }

    println!("{}", lcm_set.len());
}
