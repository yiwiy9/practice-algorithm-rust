use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc416/tasks/abc416_e
/// https://atcoder.jp/contests/abc416/editorial/13536
/// → アルゴリズムドリブンに考えるのではなく、こういうのを適切に考察できるようになってほしい
/// → 全く思い付かなかったが、ワーシャルフロイドに対する辺追加は O(N^2) で可能
///
/// 1. 道路と空港の追加がない場合
///    => ワーシャルフロイド法で O(N^3)
///
/// 2. クエリで道路の追加がある場合
///    => 全頂点対の最短距離の再計算を O(N^2) でできる。
///    => x,y を結ぶコスト t の辺が追加されたとき、頂点 i,j 間の最短経路は 以下のいづれかである。
///       - 変化しない
///       - i から x に行き、新たな辺を通って y へ、その後 j へ行く
///       - i から y に行き、新たな辺を通って x へ、その後 j へ行く
///
/// 3. クエリで空港の追加がある場合
///    => 2と同じようにやると、空港の数だけ辺の追加が必要なので、最悪計算量が O(N^4) になる。
///    => 空港がある街同士を直接つなぐのではなく、「上空」を表す頂点を追加すると、同じく全頂点対の最短距離の再計算を O(N^2) でできる。
///       - 空港のある街からは「上空」へ時間 T で移動可能
///       - 「上空」からは空港のある街へ時間 0 で移動可能
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
        k: usize,
        t: usize,
        d: [Usize1; k],
        q: usize,
    }

    let mut dp = vec![vec![INF; n + 1]; n + 1];
    for &(a, b, c) in &abc {
        dp[a][b] = dp[a][b].min(c);
        dp[b][a] = dp[b][a].min(c);
    }
    for i in 0..n + 1 {
        dp[i][i] = 0;
    }
    for &d_i in &d {
        dp[d_i][n] = t;
        dp[n][d_i] = 0;
    }

    // ワーシャルフロイド法
    for k in 0..n + 1 {
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                    tt: usize,
                }

                dp[x][y] = dp[x][y].min(tt);
                dp[y][x] = dp[y][x].min(tt);

                for i in 0..n + 1 {
                    for j in 0..n + 1 {
                        dp[i][j] = dp[i][j].min(dp[i][x] + dp[x][y] + dp[y][j]);
                        dp[i][j] = dp[i][j].min(dp[i][y] + dp[y][x] + dp[x][j]);
                    }
                }
            }
            2 => {
                input! {
                    x: Usize1,
                }

                dp[x][n] = dp[x][n].min(t);
                dp[n][x] = 0;

                for i in 0..n + 1 {
                    for j in 0..n + 1 {
                        dp[i][j] = dp[i][j].min(dp[i][x] + dp[x][n] + dp[n][j]);
                        dp[i][j] = dp[i][j].min(dp[i][n] + dp[n][x] + dp[x][j]);
                    }
                }
            }
            3 => {
                let mut ans = 0;

                for i in 0..n {
                    for j in 0..n {
                        ans += if dp[i][j] == INF { 0 } else { dp[i][j] };
                    }
                }

                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
