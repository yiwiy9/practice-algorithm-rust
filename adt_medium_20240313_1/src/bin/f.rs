use itertools::Itertools as _;
use proconio::input;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut box_map_vec = vec![BTreeMap::new(); n + 1];
    let mut card_map_vec = vec![BTreeSet::new(); 200_010];

    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                box_map_vec[j]
                    .entry(i)
                    .and_modify(|cur| *cur += 1)
                    .or_insert(1);
                card_map_vec[i].insert(j);
            }
            2 => {
                input! {
                    i: usize,
                }
                let mut ans = vec![];
                box_map_vec[i].iter().for_each(|(&num, &cnt)| {
                    for _ in 0..cnt {
                        ans.push(num);
                    }
                });
                println!("{}", ans.iter().join(" "));
            }
            3 => {
                input! {
                    i: usize,
                }
                let mut ans = vec![];
                card_map_vec[i].iter().for_each(|(&num)| {
                    ans.push(num);
                });
                println!("{}", ans.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
