use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc394/tasks/abc394_e
/// https://atcoder.jp/contests/abc394/editorial/12279
/// => 辺を頂点とみなせるか（ c + 回文 + c = 回文 はそれはそう ）
/// => 頂点自身は情報を持っていなくて、辺のみが情報を持っている
/// => 頂点自身がどの順番で訪れられたかではなく、辺がどの順番で使われたかが重要
/// => 辺を頂点捉えて、N^2頂点のグラフを考える
/// => そうすると、始点S を追加して、Sから各頂点への最短経路が答えである問題に置き換えられる
///
/// - 各i各 i について S から (i,i) へ向かう重み 0 の辺
/// - C(i,j) = - なる (i,j) について S から (i,j) へ向かう重み 1 の辺
/// - C(k,i) = C(j,l) ≠ - を満たす (i,j,k,l) について (i,j) から (k,l) へ向かう重み 2 の辺
///
/// 一見思いつくのが無理そうな感じだが、実はやりたいことは合っていた
/// => 回文の長さを徐々に大きくしていって、できるところまで続ける
/// => 確かにコレはBFSで解けそう（終了条件どうなるんやろで悩んでた）
/// => 辺を頂点とみなしたら、BFSになるね！
fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }

    let mut a = vec![vec![INF; n]; n];
    let mut que = std::collections::VecDeque::new();
    for i in 0..n {
        a[i][i] = 0;
        que.push_back((i, i));
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == INF && c[i][j] != '-' {
                a[i][j] = 1;
                que.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = que.pop_front() {
        for k in 0..n {
            for l in 0..n {
                if c[k][i] != '-' && c[k][i] == c[j][l] && a[k][l] == INF {
                    a[k][l] = a[i][j] + 2;
                    que.push_back((k, l));
                }
            }
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|row| row
                .iter()
                .map(|&a_ij| if a_ij == INF { -1 } else { a_ij as i64 })
                .join(" "))
            .join("\n")
    );
}
