use proconio::input;

/// https://atcoder.jp/contests/joisc2008/tasks/joisc2008_committee
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   人間関係の木を根付き木として見て、各人 v について「v を必ず含み、v の部分木内で作れる連結委員会の最大和」を持てばよい。
/// - それについて、何が分かれば答えになる？
///   各 v を最高位の人として含む連結委員会の最大和が分かれば、その最大値が答えになる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   親へ返すとき、子側の最大和が負なら v を含む委員会に加えると損なので、その子部分木は捨ててよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   DFS の帰りがけに `cur = a[v] + sum(max(0, child_dp))` を計算し、`ans` を `cur` で更新してから `cur` を親へ返す。
///
/// AC: 22分
fn main() {
    input! {
        n: usize,
        sa: [(usize, i64); n],
    }

    let mut graph = vec![vec![]; n];
    let mut a = vec![0; n];
    for (i, &(s_i, a_i)) in sa.iter().enumerate() {
        a[i] = a_i;
        if s_i == 0 {
            continue;
        }
        graph[i].push(s_i - 1);
        graph[s_i - 1].push(i);
    }

    let mut ans = -(1 << 60);
    dfs(&graph, &a, &mut ans, n, 0);

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<usize>>, a: &[i64], ans: &mut i64, par_v: usize, v: usize) -> i64 {
    let mut cur = a[v];
    for &next_v in &graph[v] {
        if next_v == par_v {
            continue;
        }
        let res = dfs(graph, a, ans, v, next_v);
        if res > 0 {
            cur += res;
        }
    }

    if cur > *ans {
        *ans = cur;
    }
    cur
}
