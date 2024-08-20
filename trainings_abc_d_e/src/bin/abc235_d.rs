use proconio::input;
const MAX_N: usize = 1_000_000;

/**
 * https://atcoder.jp/contests/abc235/tasks/abc235_d
 * https://atcoder.jp/contests/abc235/editorial/3255
 * dist管理の最短経路は、BFSで求める
 *
 * chars.rotate_right(1) で、右に1つずつずらす
 * chars_to_decimal, decimal_to_charsだとエラーが出た
 */
fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let dist = bfs(a);

    println!("{}", dist[n]);
}

pub fn bfs(a: usize) -> Vec<i64> {
    let mut dist = vec![-1; MAX_N];
    let mut que = std::collections::VecDeque::new();
    dist[1] = 0;
    que.push_back(1);
    while let Some(x) = que.pop_front() {
        let x1 = x * a;
        if x1 < MAX_N && dist[x1] == -1 {
            dist[x1] = dist[x] + 1;
            que.push_back(x1);
        }

        if x >= 10 && x % 10 != 0 {
            let mut x_chars: Vec<char> = x.to_string().chars().collect();
            x_chars.rotate_right(1);
            let x2 = x_chars.iter().collect::<String>().parse::<usize>().unwrap();
            if x2 < MAX_N && dist[x2] == -1 {
                dist[x2] = dist[x] + 1;
                que.push_back(x2);
            }
        }
    }
    dist
}
