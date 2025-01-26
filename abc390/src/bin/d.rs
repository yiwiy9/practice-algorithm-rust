use proconio::input;
use std::collections::HashSet;

/// https://atcoder.jp/contests/abc390/tasks/abc390_d
/// https://atcoder.jp/contests/abc390/editorial/12045
///
/// 【グループ分けの全列挙】
/// N 個のものをグループに分ける方法（グループの順番、グループ内の順番は区別しない）の個数はベル数 B(N) としてしられています。
/// ベル数は N とともに増加し、N=12 においては B(12)=4213597(∼4×10 6) 通りとなるため、
/// 今回の制約 (2≤N≤12) の下では、全列挙することが可能です。
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = std::collections::HashSet::new();
    dfs_partition(n, &a, &mut set, 0, 0, &mut vec![0; n + 1], 0);

    println!("{}", set.len());
}

fn dfs_partition(
    n: usize,
    a: &Vec<usize>,
    set: &mut HashSet<usize>,
    group_count: usize,
    idx: usize,
    group_sum: &mut Vec<usize>,
    mut all_xor: usize,
) {
    for g_idx in 0..=group_count {
        all_xor ^= group_sum[g_idx];
        group_sum[g_idx] += a[idx];
        all_xor ^= group_sum[g_idx];

        if idx == n - 1 {
            set.insert(all_xor);
        } else if g_idx < group_count {
            dfs_partition(n, a, set, group_count, idx + 1, group_sum, all_xor);
        } else {
            dfs_partition(n, a, set, group_count + 1, idx + 1, group_sum, all_xor);
        }

        all_xor ^= group_sum[g_idx];
        group_sum[g_idx] -= a[idx];
        all_xor ^= group_sum[g_idx];
    }
}
