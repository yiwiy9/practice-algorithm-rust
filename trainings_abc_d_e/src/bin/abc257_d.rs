use std::collections::BTreeSet;

use proconio::input;

/**
 * https://atcoder.jp/contests/abc257/tasks/abc257_d
 * https://blog.hamayanhamayan.com/entry/2022/06/25/231725
 * ↑の解説の問題に対するアプローチがものすごく参考になる
 *
 * 1. いつものように全探索解法がないか探してみる。全探索できそうなのはSの値か、始点となるジャンプ台くらいな感じがする。
 * 2. Sの値を増やしていくとジャンプすることができる遷移がどんどん増えていくことになる。
 *    そして、大事なのが、Sの値を増やしても前使えた遷移が使えなくなることがないということ。
 *    => Sの値をどんどん増やしていくと、あるSの値を境に高橋君の目的が達成される
 *    => 二分探索
 * 3. ここまでこれば、あとは各S毎に全点到達可能かを探索すれば良い
 *    => ここで次は始点について全探索する必要がある
 *    => その後の到達可能かの探索はBFSで行える
 */
fn main() {
    input! {
        n: usize,
        xyp: [(i64,i64,i64); n],
    }

    let mut left = -1;
    let mut right = 4_000_000_001;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check_all_bfs(n, &xyp, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

fn check_all_bfs(n: usize, xyp: &[(i64, i64, i64)], s: i64) -> bool {
    for i in 0..n {
        let mut visited = BTreeSet::new();
        let mut que = std::collections::VecDeque::new();
        visited.insert(i);
        que.push_back(i);
        while let Some(u) = que.pop_front() {
            for v in 0..n {
                if visited.contains(&v) || !can_move(xyp, s, u, v) {
                    continue;
                }
                visited.insert(v);
                que.push_back(v);
            }
        }

        if visited.len() == n {
            return true;
        }
    }

    false
}

fn can_move(xyp: &[(i64, i64, i64)], s: i64, from: usize, to: usize) -> bool {
    let (x1, y1, p1) = xyp[from];
    let (x2, y2, _) = xyp[to];
    s * p1 >= (x1 - x2).abs() + (y1 - y2).abs()
}
