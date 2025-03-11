use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc396/tasks/abc396_e
/// https://atcoder.jp/contests/abc396/editorial/12390
///
/// 各項で XOR の値がわかっている状態で、その数列の和の最小値を求める
/// => 固定する値をどのように決めれば、全体の和が最小になるか
/// => 固定する値の全探索は到底間に合わないが、ビットごとに考えると２進数桁数分の計算で済む
///
/// XOR は bitごとに独立しているので、
/// 各ビットごとにスタートを 0 or 1 に固定した場合で試して、全体で1の数が少ない方を選ぶ
fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(Usize1, Usize1, i64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(x, y, z) in &xyz {
        graph[x].push((y, z));
        graph[y].push((x, z));
    }

    let mut ans = vec![0; n];
    let mut value = vec![-1; n];
    for v in 0..n {
        if value[v] != -1 {
            continue;
        }

        value[v] = 0;
        let mut connected = vec![];
        let res = dfs(&graph, &mut value, &mut connected, v);
        if !res {
            println!("-1");
            return;
        }

        for bit_digit in 0..30 {
            let one_cnt = connected
                .iter()
                .filter(|&&v| value[v] & (1 << bit_digit) != 0)
                .count();

            if one_cnt * 2 < connected.len() {
                for &v in &connected {
                    if value[v] & (1 << bit_digit) != 0 {
                        ans[v] |= 1 << bit_digit;
                    }
                }
            } else {
                for &v in &connected {
                    if value[v] & (1 << bit_digit) == 0 {
                        ans[v] |= 1 << bit_digit;
                    }
                }
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<(usize, i64)>>,
    value: &mut Vec<i64>,
    connected: &mut Vec<usize>,
    v: usize,
) -> bool {
    connected.push(v);
    for &(next_v, w) in &graph[v] {
        if value[next_v] != -1 {
            if value[next_v] != value[v] ^ w {
                return false;
            }
            continue;
        }

        value[next_v] = value[v] ^ w;
        let res = dfs(graph, value, connected, next_v);
        if !res {
            return false;
        }
    }
    true
}
