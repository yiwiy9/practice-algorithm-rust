use std::collections::BTreeMap;

use proconio::input;

/// 観察
/// - 選ぶ回数が多いほど、桁数は増えるので良い
/// - 使うお金が同じなら、大きい数字を選んだ方が良い
fn main() {
    input! {
        n: usize,
        c: [usize; 9],
    }

    let mut dp = vec![BTreeMap::new(); n + 1];
    for (j, &c_j) in c.iter().enumerate() {
        if c_j > n {
            continue;
        }
        let mut map = BTreeMap::new();
        map.insert(j + 1, 1);
        if is_grater_equal(&map, &dp[c_j]) {
            dp[c_j] = map;
        }
    }

    for i in 0..n {
        if dp[i].is_empty() {
            continue;
        }
        for (j, &c_j) in c.iter().enumerate() {
            let next_i = i + c_j;
            if next_i > n {
                continue;
            }

            let mut next_map = dp[i].clone();
            next_map.entry(j + 1).and_modify(|v| *v += 1).or_insert(1);

            if is_grater_equal(&next_map, &dp[next_i]) {
                dp[next_i] = next_map;
            }
        }
    }

    let mut ans_dp = BTreeMap::new();
    for i in 0..=n {
        if is_grater_equal(&dp[i], &ans_dp) {
            ans_dp = dp[i].clone();
        }
    }

    let ans = ans_dp
        .iter()
        .rev()
        .map(|(&k, &v)| {
            let mut s = String::new();
            for _ in 0..v {
                s.push_str(&k.to_string());
            }
            s
        })
        .collect::<Vec<_>>();

    println!("{}", ans.join(""));
}

fn is_grater_equal(a: &BTreeMap<usize, usize>, b: &BTreeMap<usize, usize>) -> bool {
    let a_len = a.iter().map(|(_, &v)| v).sum::<usize>();
    let b_len = b.iter().map(|(_, &v)| v).sum::<usize>();
    if a_len > b_len {
        return true;
    } else if a_len < b_len {
        return false;
    }

    for i in (1..=9).rev() {
        let a_i = a.get(&i).unwrap_or(&0);
        let b_i = b.get(&i).unwrap_or(&0);
        if a_i > b_i {
            return true;
        } else if a_i < b_i {
            return false;
        }
    }
    true
}
