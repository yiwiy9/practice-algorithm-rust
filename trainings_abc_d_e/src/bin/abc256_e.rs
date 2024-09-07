// use proconio::{input, marker::Usize1};
// use std::{
//     cmp::Reverse,
//     collections::{BinaryHeap, HashSet},
// };

// fn main() {
//     input! {
//         n: usize,
//         x: [Usize1; n],
//         c: [usize; n],
//     }

//     let mut hate = vec![0; n];
//     for i in 0..n {
//         hate[x[i]] += c[i];
//     }

//     let mut heap = BinaryHeap::new();
//     for (i, &h) in hate.iter().enumerate() {
//         heap.push(Reverse((h, i)));
//     }

//     let mut ans = 0;
//     let mut used = HashSet::new();
//     while let Some(Reverse((h, i))) = heap.pop() {
//         if used.contains(&i) {
//             continue;
//         }

//         ans += h;
//         used.insert(i);

//         hate[x[i]] -= c[i];
//         heap.push(Reverse((hate[x[i]], x[i])));
//     }

//     println!("{}", ans);
// }

/**
 * ↑よくACできたけど、別解あり
 * https://atcoder.jp/contests/abc256/tasks/abc256_e
 * https://atcoder.jp/contests/abc256/editorial/4135
 *
 * グラフとして捉えると、
 *   - N頂点N辺の自己辺を持たない有向グラフGがある。各頂点の出次数は 1 で、頂点 i から出る辺は Xi へ向かっている。
 *
 * このようにすべての頂点の出次数が 1 の有向グラフを Functional Graph と呼び、以下の性質を持つ
 *   - G の連結成分が K 個あって C1,C2,…,CK とする。このとき、各連結成分には閉路が 1 つだけ存在する。
 *     - 書いたらわかるが、丸と矢印のセットを書いていくと必ず閉路が一つできる
 *
 * つまり、連結成分の閉路の部分の開始を自由に選べるので、Min(ci,...,cj) が最小になるように選べばよい
 * 閉路の外の枝葉の部分は必ず閉路に向かうので、閉路内の点を選ぶ前に選ぶことでコストを0にできる
 * => 連結成分ごとの閉路のMIN(C)の和が答え
 */
use ac_library::Dsu;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [usize; n],
    }
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for i in 0..n {
        if !dsu.same(i, x[i]) {
            dsu.merge(i, x[i]);
            continue;
        }

        // サイクルができたら、サイクル内の最小コストを選ぶ
        let mut min_cost = c[i];
        let mut v = x[i];
        while v != i {
            min_cost = min_cost.min(c[v]);
            v = x[v];
        }
        ans += min_cost;
    }
    println!("{}", ans);
}
