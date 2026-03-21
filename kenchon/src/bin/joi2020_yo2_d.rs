use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        m: usize,
        r: usize,
    }

    // テンキー上の隣接関係
    let graph = vec![
        vec![1],
        vec![0, 2, 4],
        vec![1, 3, 5],
        vec![2, 6],
        vec![1, 5, 7],
        vec![2, 4, 6, 8],
        vec![3, 5, 9],
        vec![4, 8],
        vec![5, 7, 9],
        vec![6, 8],
    ];

    let inf = 1usize << 60;
    // dist[pos][rem] = 今 pos にいて、入力した数の余りが rem になる最小操作回数
    let mut dist = vec![vec![inf; m]; 10];
    let mut que = VecDeque::new();

    // 初期状態: 0 の上にいて、まだ何も入力していない
    // R >= 1 なので、この状態を rem = 0 として持っても答えは壊れない
    dist[0][0] = 0;
    que.push_back((0usize, 0usize));

    while let Some((pos, rem)) = que.pop_front() {
        let d = dist[pos][rem];

        // 1. 今いる数字 pos を押す
        let next_rem = (rem * 10 + pos) % m;
        if dist[pos][next_rem] == inf {
            dist[pos][next_rem] = d + 1;
            que.push_back((pos, next_rem));
        }

        // 2. 隣接する数字へ移動する
        for &to in &graph[pos] {
            if dist[to][rem] == inf {
                dist[to][rem] = d + 1;
                que.push_back((to, rem));
            }
        }
    }

    // 余り r を作れる最小値を全位置から拾う
    let mut ans = inf;
    for pos in 0..10 {
        ans = ans.min(dist[pos][r]);
    }

    println!("{}", ans);
}
