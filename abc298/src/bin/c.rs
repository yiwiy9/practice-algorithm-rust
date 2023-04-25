use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut box_vec = vec![BTreeMap::new(); n + 1];
    let mut card_vec = vec![BTreeMap::new(); 200001];

    for _q in 0..q {
        input! {
            num: usize,
            i: usize,
        }
        match num {
            1 => {
                input! {
                    j: usize,
                }
                box_vec[j].entry(i).and_modify(|cur| *cur += 1).or_insert(1);
                card_vec[i]
                    .entry(j)
                    .and_modify(|cur| *cur += 1)
                    .or_insert(1);
            }
            2 => {
                let mut is_first = true;
                for (k, &v) in &box_vec[i] {
                    for _v in 0..v {
                        if !is_first {
                            print!(" ");
                        }
                        print!("{}", k);
                        is_first = false;
                    }
                }
                println!()
            }
            3 => {
                let mut is_first = true;
                for (k, &_v) in &card_vec[i] {
                    if !is_first {
                        print!(" ");
                    }
                    print!("{}", k);
                    is_first = false;
                }
                println!()
            }
            _ => unreachable!(),
        }
    }
}
