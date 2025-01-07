use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

/// https://atcoder.jp/contests/adt_all_20250107_1/tasks/abc224_e
/// https://atcoder.jp/contests/adt_all_20250107_1/editorial/2814
/// 当初の解法は最悪時間計算量が Θ(N^2) であるため、TLE が発生する。
///
/// dp[i]:=（現在、コマがマス (ri, ci) に置かれているとき、それ以降に高橋君がコマの移動を行うことができる回数の最大値）
/// 緩和の順番は先頭からではなく、a_i の降順に行う。
/// そして、緩和をO(1)で行うために、行と列それぞれについて、現時点での最大値を管理する配列を用意する。
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rca: [(Usize1, Usize1, usize); n],
    }

    let mut a_map = BTreeMap::new();
    for (i, &(_, _, a)) in rca.iter().enumerate() {
        a_map.entry(a).or_insert(vec![]).push(i);
    }

    let mut row_max = vec![0; h];
    let mut col_max = vec![0; w];
    let mut dp = vec![0; n];
    for (_, idx_vec) in a_map.iter().rev() {
        for &idx in idx_vec {
            let (r, c, _) = rca[idx];
            dp[idx] = row_max[r].max(col_max[c]);
        }
        for &idx in idx_vec {
            let (r, c, _) = rca[idx];
            row_max[r] = row_max[r].max(dp[idx] + 1);
            col_max[c] = col_max[c].max(dp[idx] + 1);
        }
    }

    println!("{}", dp.iter().join("\n"));
}

// use proconio::input;
// use std::collections::{BTreeMap, BTreeSet};

// fn main() {
//     input! {
//         _: usize,
//         _: usize,
//         n: usize,
//         rca: [(usize, usize, usize); n],
//     }

//     let mut row_map = BTreeMap::new();
//     let mut col_map = BTreeMap::new();
//     for &(r, c, a) in &rca {
//         row_map.entry(r).or_insert(BTreeSet::new()).insert((a, c));
//         col_map.entry(c).or_insert(BTreeSet::new()).insert((a, r));
//     }

//     let mut memo = BTreeMap::new();
//     for &(r, c, a) in &rca {
//         println!("{}", dfs(&row_map, &col_map, &mut memo, r, c, a));
//     }
// }

// pub fn dfs(
//     row_map: &BTreeMap<usize, BTreeSet<(usize, usize)>>,
//     col_map: &BTreeMap<usize, BTreeSet<(usize, usize)>>,
//     memo: &mut BTreeMap<(usize, usize), usize>,
//     r: usize,
//     c: usize,
//     a: usize,
// ) -> usize {
//     if let Some(&res) = memo.get(&(r, c)) {
//         return res;
//     }

//     let mut res = 0;
//     if let Some(row_col_set) = row_map.get(&r) {
//         for &(a_i, c_i) in row_col_set.range((a + 1, 0)..) {
//             res = res.max(dfs(row_map, col_map, memo, r, c_i, a_i) + 1);
//         }
//     }
//     if let Some(col_row_map) = col_map.get(&c) {
//         for &(a_i, r_i) in col_row_map.range((a + 1, 0)..) {
//             res = res.max(dfs(row_map, col_map, memo, r_i, c, a_i) + 1);
//         }
//     }

//     memo.insert((r, c), res);
//     res
// }
