use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s_x: i64,
        s_y: i64,
        xy: [(i64,i64); n],
        d: [(char,i64); m],
    }

    let mut x_map = std::collections::HashMap::new();
    let mut y_map = std::collections::HashMap::new();
    for (x, y) in xy {
        x_map.entry(x).or_insert(BTreeSet::new()).insert(y);
        y_map.entry(y).or_insert(BTreeSet::new()).insert(x);
    }

    let mut ans = 0;
    let mut x = s_x;
    let mut y = s_y;
    for (c, d_i) in d {
        match c {
            'U' => {
                if !x_map.contains_key(&x) {
                    y += d_i;
                    continue;
                }
                let y_vec = x_map.get_mut(&x).unwrap();
                let target_y_list = y_vec.range(y..=y + d_i).cloned().collect::<Vec<_>>();
                for target_y in &target_y_list {
                    ans += 1;
                    if let Some(v) = y_map.get_mut(target_y) {
                        v.remove(&x);
                    }
                    y_vec.remove(target_y);
                }
                y += d_i;
            }
            'D' => {
                if !x_map.contains_key(&x) {
                    y -= d_i;
                    continue;
                }
                let y_vec = x_map.get_mut(&x).unwrap();
                let target_y_list = y_vec.range((y - d_i)..=y).cloned().collect::<Vec<_>>();
                for target_y in &target_y_list {
                    ans += 1;
                    if let Some(v) = y_map.get_mut(target_y) {
                        v.remove(&x);
                    }
                    y_vec.remove(target_y);
                }
                y -= d_i;
            }
            'L' => {
                if !y_map.contains_key(&y) {
                    x -= d_i;
                    continue;
                }
                let x_vec = y_map.get_mut(&y).unwrap();
                let target_x_list = x_vec.range((x - d_i)..=x).cloned().collect::<Vec<_>>();
                for target_x in &target_x_list {
                    ans += 1;
                    if let Some(v) = x_map.get_mut(target_x) {
                        v.remove(&y);
                    }
                    x_vec.remove(target_x);
                }
                x -= d_i;
            }
            'R' => {
                if !y_map.contains_key(&y) {
                    x += d_i;
                    continue;
                }
                let x_vec = y_map.get_mut(&y).unwrap();
                let target_x_list = x_vec.range(x..=x + d_i).cloned().collect::<Vec<_>>();
                for target_x in &target_x_list {
                    ans += 1;
                    if let Some(v) = x_map.get_mut(target_x) {
                        v.remove(&y);
                    }
                    x_vec.remove(target_x);
                }
                x += d_i;
            }
            _ => unreachable!(),
        }
    }

    println!("{} {} {}", x, y, ans);
}
