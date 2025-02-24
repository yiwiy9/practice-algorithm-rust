use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc394/tasks/abc394_e
/// https://atcoder.jp/contests/abc394/editorial/12279
///
/// 「cを英小文字、Sを回文とすると c,S,cをこの順に結合して得られる文字列は回文になる」
/// この情報から出発すると、ある回文のパスの両端に同じ文字のパスを追加しても回文になることがわかるので、
/// 各パス(i,j)を頂点とする N^2 グラフを考えれば上手くいきそうという発想が生まれる
/// => あとは、便宜上の始点Sを加えて、各パス(i,j)への最短経路問題として定式化できる
///
/// ** N^2グラフとみなすのと、便宜上の始点Sを加えることを思い付くのが難しい **
///
/// ここで、各パス(i,j)間の辺は、以下のような重み付き有向辺として表現できる
/// 1. 各 i について S から (i,i)へ向かう重み 0 の辺
///     - 長さ 0 の回文
/// 2. C_ij != '-' なる (i,j) について S から (i,j) へ向かう重み 1 の辺
///     - 長さ 1 の回文
/// 3. C_ki != '-', C_jl != '-', C_ki = C_jl を満たす (i,j,k,l) について (i,j) から (k,l) へ向かう重み 2 の辺
///     - 両端に同じ文字を追加することで回文の長さを 2 増やす。(k,l)の回文パスが更新される
///
/// あとは、始点Sから各パス(i,j)に対する辺1,2を先に処理することで、辺3に対するBFSを行えば良いだけとなる
///
///
fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }

    let mut a = vec![vec![INF; n]; n];
    let mut que = VecDeque::new();

    // 辺1
    for i in 0..n {
        a[i][i] = 0;
        que.push_back((i, i));
    }

    // 辺2
    for i in 0..n {
        for j in 0..n {
            if i != j && c[i][j] != '-' {
                a[i][j] = 1;
                que.push_back((i, j));
            }
        }
    }

    // 辺3
    while let Some((i, j)) = que.pop_front() {
        for k in 0..n {
            for l in 0..n {
                if c[k][i] != '-' && c[j][l] != '-' && c[k][i] == c[j][l] && a[k][l] == INF {
                    // BFSなので、最初に見つかったパスが最短経路
                    a[k][l] = a[i][j] + 2;
                    que.push_back((k, l));
                }
            }
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|a_i| a_i
                .iter()
                .map(|&a_ij| if a_ij == INF { -1 } else { a_ij as i64 })
                .join(" "))
            .join("\n")
    );
}
