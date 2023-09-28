use proconio::input;
use std::collections::BTreeMap;

const N: i64 = 1048576;

fn main() {
    input! {
        q: usize,
        tx: [(usize,i64); q],
    }

    let mut map = BTreeMap::new();
    for (t, x) in tx {
        let h = x % N;
        match t {
            1 => {
                rec(&mut map, h, x);
            }
            2 => {
                if !map.contains_key(&h) {
                    println!("{}", -1);
                } else {
                    let v = map.get(&h).unwrap();
                    println!("{}", v.1);
                }
            }
            _ => unreachable!(),
        }
    }
}

fn rec(map: &mut BTreeMap<i64, (i64, i64)>, h: i64, x: i64) -> i64 {
    if !map.contains_key(&h) {
        let end_h = (h + 1) % N;
        map.insert(h, (end_h, x));
        return end_h;
    }

    let &(cur_h, cur_x) = map.get(&h).unwrap();
    let end_h = rec(map, cur_h, x);

    map.insert(h, (end_h, cur_x));
    end_h
}
