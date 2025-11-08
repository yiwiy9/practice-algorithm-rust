use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            op: u8
        }
        match op {
            1 => {
                input! {
                    x: usize,
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                    mut k: usize,
                }

                let mut before = x + 1;
                loop {
                    if let Some((&num, &cnt)) = map.range(..before).next_back() {
                        if cnt >= k {
                            println!("{}", num);
                            break;
                        } else {
                            k -= cnt;
                            before = num;
                        }
                    } else {
                        println!("-1");
                        break;
                    }
                }
            }
            3 => {
                input! {
                    x: usize,
                    mut k: usize,
                }

                let mut before = x - 1;
                loop {
                    if let Some((&num, &cnt)) = map.range(before + 1..).next() {
                        if cnt >= k {
                            println!("{}", num);
                            break;
                        } else {
                            k -= cnt;
                            before = num;
                        }
                    } else {
                        println!("-1");
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
