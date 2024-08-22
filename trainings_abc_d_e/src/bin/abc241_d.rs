use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
    }

    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            op: usize,
            x: i64,
        }

        match op {
            1 => {
                map.entry(x).and_modify(|v| *v += 1).or_insert(1);
            }
            2 => {
                input! {
                    mut k: i64,
                }
                let mut iter = map.range(..=x);
                loop {
                    if let Some((&num, &cnt)) = iter.next_back() {
                        k -= cnt;
                        if k <= 0 {
                            println!("{}", num);
                            break;
                        }
                    } else {
                        println!("{}", -1);
                        break;
                    }
                }
            }
            3 => {
                input! {
                    mut k: i64,
                }
                let mut iter = map.range(x..);
                loop {
                    if let Some((&num, &cnt)) = iter.next() {
                        k -= cnt;
                        if k <= 0 {
                            println!("{}", num);
                            break;
                        }
                    } else {
                        println!("{}", -1);
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
