use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; n],
    }

    let mut zoo = vec![vec![0; m]; n];
    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }
        for j in 0..k {
            zoo[a[j]][i] = 1;
        }
    }

    let mut dp = HashMap::new();
    dp.insert(vec![0; m], 0);

    for i in 0..n {
        let mut new_dp: HashMap<Vec<i32>, usize> = HashMap::new();
        for (key, value) in &dp {
            new_dp
                .entry(key.clone())
                .and_modify(|cur| *cur = (*cur).min(*value))
                .or_insert(*value);

            let mut new_key_1 = key.clone();
            for j in 0..m {
                new_key_1[j] += zoo[i][j];
            }
            new_key_1 = new_key_1
                .iter()
                .map(|&x| if x > 2 { 2 } else { x })
                .collect();
            let new_value_1 = value + c[i];
            new_dp
                .entry(new_key_1)
                .and_modify(|cur| *cur = (*cur).min(new_value_1))
                .or_insert(new_value_1);

            let mut new_key_2 = key.clone();
            for j in 0..m {
                new_key_2[j] += 2 * zoo[i][j];
            }
            new_key_2 = new_key_2
                .iter()
                .map(|&x| if x > 2 { 2 } else { x })
                .collect();
            let new_value_2 = value + 2 * c[i];
            new_dp
                .entry(new_key_2)
                .and_modify(|cur| *cur = (*cur).min(new_value_2))
                .or_insert(new_value_2);
        }
        dp = new_dp;
    }

    println!("{}", dp.get(&vec![2; m]).unwrap_or(&0));
}
