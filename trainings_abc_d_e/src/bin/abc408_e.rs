use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc408/tasks/abc408_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   答えの bit ごとの話にする
/// - それについて、何が分かれば答えになる？
///   各 bit を 0 のままにできるかが分かれば答えになる
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   上位 bit が 0 の方が、下位 bit がどうであっても常に小さい
///   したがって大きい bit から順に、立てなくて済むなら立てない貪欲でよい
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   大きい bit から見て、今その bit を 0 に固定した mask で 1-N が到達可能か判定する
///   到達不能ならその bit を答えに立て
///
/// AC: 29分
fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut ans: usize = 0;
    for d in (0..30).rev() {
        let cur = ans | ((1 << d) - 1);
        let mut seen = vec![false; n];
        dfs(&graph, cur, &mut seen, 0);
        if !seen[n - 1] {
            ans |= 1 << d;
        }
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, cur: usize, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &(next_v, w) in &graph[v] {
        if seen[next_v] {
            continue;
        }
        let next_w = cur | w;
        if cur != next_w {
            continue;
        }
        dfs(graph, cur, seen, next_v);
    }
}
