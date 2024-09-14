// use proconio::{input, marker::Usize1};

// fn main() {
//     input! {
//         n: usize,
//         a: [Usize1; n],
//     }

//     let mut graph = vec![vec![]; n];
//     for i in 0..n {
//         graph[a[i]].push(i);
//     }

//     let mut seen = vec![false; n];
//     let mut finished = vec![false; n];
//     let mut cycle_stack = vec![];
//     for i in 0..n {
//         if seen[i] {
//             continue;
//         }

//         dfs(&graph, &mut seen, &mut finished, &mut cycle_stack, i);
//     }

//     println!("{}", cycle_stack.len());
// }

// pub fn dfs(
//     graph: &Vec<Vec<usize>>,
//     seen: &mut Vec<bool>,
//     finished: &mut Vec<bool>,
//     cycle_stack: &mut Vec<usize>,
//     v: usize,
// ) -> bool {
//     seen[v] = true;
//     cycle_stack.push(v);
//     for &next_v in &graph[v] {
//         if finished[next_v] {
//             continue;
//         }

//         if seen[next_v] && !finished[next_v] {
//             return true;
//         }

//         if dfs(graph, seen, finished, cycle_stack, next_v) {
//             return true;
//         }
//     }
//     cycle_stack.pop();
//     finished[v] = true;
//     false
// }

/*
 * 【別解: Functional Graph】abc256_e.rs
 * グラフとして捉えると、
 *   - N頂点N辺の自己辺を持たない有向グラフGがある。各頂点の出次数は 1 で、頂点 i から出る辺は Xi へ向かっている。
 *
 * このようにすべての頂点の出次数が 1 の有向グラフを Functional Graph と呼び、以下の性質を持つ
 *   - G の連結成分が K 個あって C1,C2,…,CK とする。このとき、各連結成分には閉路が 1 つだけ存在する。
 *     - 書いたらわかるが、丸と矢印のセットを書いていくと必ず閉路が一つできる
 */
use ac_library::Dsu;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for i in 0..n {
        if !dsu.same(i, a[i]) {
            dsu.merge(i, a[i]);
            continue;
        }

        // サイクルができたら、サイクル内の頂点を数える
        let mut cnt = 1;
        let mut v = a[i];
        while v != i {
            cnt += 1;
            v = a[v];
        }
        ans += cnt;
    }
    println!("{}", ans);
}
