use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        t: String,
        n: usize,
    }

    let mut s = vec![];
    for _ in 0..n {
        input! {
            a_i: usize,
            s_i: [String; a_i],
        }
        s.push(s_i);
    }

    let t_chars = t.chars().collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    map.insert(String::new(), 0_i64);

    for s_i in s {
        let mut new_map = BTreeMap::new();
        for (k, v) in map {
            for s_ij in &s_i {
                let mut k_clone = k.clone();
                k_clone.push_str(s_ij);

                if k_clone.len() > t.len() {
                    continue;
                }

                let mut ok = true;
                for (i, c) in k_clone.chars().enumerate() {
                    if c != t_chars[i] {
                        ok = false;
                        break;
                    }
                }

                if !ok {
                    continue;
                }

                let mut v_next = v + 1;
                new_map
                    .entry(k_clone)
                    .and_modify(|cur: &mut i64| *cur = *cur.min(&mut v_next))
                    .or_insert(v + 1);
            }

            let mut v_next = v;
            new_map
                .entry(k)
                .and_modify(|cur: &mut i64| *cur = *cur.min(&mut v_next))
                .or_insert(v);
        }
        map = new_map;
    }

    println!("{}", map.get(&t).unwrap_or(&(-1)));
}
