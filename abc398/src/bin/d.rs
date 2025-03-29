use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        r: i64,
        c: i64,
        s: Chars,
    }

    let mut r_acc = vec![0; n + 1];
    let mut c_acc = vec![0; n + 1];
    for (i, &c) in s.iter().enumerate() {
        r_acc[i + 1] = r_acc[i];
        c_acc[i + 1] = c_acc[i];
        match c {
            'S' => {
                r_acc[i + 1] += 1;
            }
            'N' => {
                r_acc[i + 1] -= 1;
            }
            'W' => {
                c_acc[i + 1] -= 1;
            }
            'E' => {
                c_acc[i + 1] += 1;
            }
            _ => {}
        }
    }

    let mut ans = vec![false; n + 1];
    let mut r_acc_map = HashMap::new();
    let mut c_acc_map = HashMap::new();
    for i in 1..=n {
        let cur_r = r_acc[i];
        let cur_c = c_acc[i];

        if cur_r == r && cur_c == c {
            ans[i] = true;
            r_acc_map.entry(cur_r).or_insert(HashSet::new()).insert(i);
            c_acc_map.entry(cur_c).or_insert(HashSet::new()).insert(i);
            continue;
        }

        let Some(r_indices) = r_acc_map.get(&(cur_r-r)) else {
            r_acc_map.entry(cur_r).or_insert(HashSet::new()).insert(i);
            c_acc_map.entry(cur_c).or_insert(HashSet::new()).insert(i);
            continue;
        };
        let Some(c_indices) = c_acc_map.get(&(cur_c-c)) else {
            r_acc_map.entry(cur_r).or_insert(HashSet::new()).insert(i);
            c_acc_map.entry(cur_c).or_insert(HashSet::new()).insert(i);
            continue;
        };

        if r_indices.len() < c_indices.len() {
            if check(r_indices, c_indices) {
                ans[i] = true;
            }
        } else {
            if check(c_indices, r_indices) {
                ans[i] = true;
            }
        }
        r_acc_map.entry(cur_r).or_insert(HashSet::new()).insert(i);
        c_acc_map.entry(cur_c).or_insert(HashSet::new()).insert(i);
    }

    println!(
        "{}",
        ans.iter()
            .skip(1)
            .map(|&x| if x { '1' } else { '0' })
            .collect::<String>()
    );
}

fn check(small_indices: &HashSet<usize>, large_indices: &HashSet<usize>) -> bool {
    for small_index in small_indices {
        if large_indices.contains(small_index) {
            return true;
        }
    }
    false
}
