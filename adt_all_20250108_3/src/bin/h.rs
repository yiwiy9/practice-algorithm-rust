use proconio::input;
use std::collections::HashMap;

/// https://atcoder.jp/contests/adt_all_20250108_3/tasks/abc322_e
/// https://atcoder.jp/contests/adt_all_20250108_3/editorial/7305
/// 全ての開発案を実行するかどうかを全通り試す O(2^NNK) 解法は間に合いません。
/// そこで、P を超えたステータスは P で打ち切ってもよいため途中でのステータスの種類は実質 O((P+1)K) 通りしかないことに注目します。
fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }

    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; k],
        }
        c.push(c_i);
        a.push(a_i);
    }

    let mut dp = HashMap::new();
    dp.insert(vec![0; k], 0);

    for i in 0..n {
        let mut next_dp = HashMap::new();

        for (state, &cost) in &dp {
            if !next_dp.contains_key(state) || next_dp[state] > cost {
                next_dp.insert(state.clone(), cost);
            }

            let mut new_state = vec![0; k];
            for j in 0..k {
                new_state[j] = p.min(state[j] + a[i][j]);
            }
            if !next_dp.contains_key(&new_state) || next_dp[&new_state] > cost + c[i] {
                next_dp.insert(new_state.clone(), cost + c[i]);
            }
        }

        dp = next_dp;
    }

    println!(
        "{}",
        if let Some(&ans) = dp.get(&vec![p; k]) {
            ans as isize
        } else {
            -1
        }
    );
}
