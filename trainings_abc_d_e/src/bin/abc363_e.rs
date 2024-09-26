use proconio::input;
use std::collections::VecDeque;

/**
 * https://atcoder.jp/contests/abc363/tasks/abc363_e
 * https://atcoder.jp/contests/abc363/editorial/10482
 * 問題の性質はわかったが、詰め方が1ミリもわからなかった
 * => 各年ごとに沈む島のキューを作って、それを1年目から順に処理し、キューに追加していく
 * => 各年ごとのキューの配列には、各島は1回しか追加されないので、O(HW)で処理できる
 */
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
    }

    let mut queues = vec![VecDeque::new(); n + 1];
    let mut is_queued = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
                if a[i][j] > n {
                    continue;
                }
                queues[a[i][j]].push_back((i, j));
                is_queued[i][j] = true;
            }
        }
    }

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let mut ans = h * w;
    for i in 1..=n {
        while let Some((x, y)) = queues[i].pop_front() {
            ans -= 1;

            for dir in 0..4 {
                let nx = x as i32 + dx[dir];
                let ny = y as i32 + dy[dir];
                if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if is_queued[nx][ny] {
                    continue;
                }
                if a[nx][ny] > n {
                    continue;
                }
                let next_i = a[nx][ny].max(i);
                queues[next_i].push_back((nx, ny));
                is_queued[nx][ny] = true;
            }
        }

        println!("{}", ans);
    }
}
