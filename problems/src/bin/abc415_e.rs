use proconio::input;
const INF: i64 = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h+w-1],
    }

    let dx: [i32; 2] = [-1, 0];
    let dy: [i32; 2] = [0, -1];

    let mut dist = vec![vec![INF; w]; h];
    let mut que = std::collections::VecDeque::new();
    dist[h - 1][w - 1] = (p[h + w - 2] - a[h - 1][w - 1]).max(0);
    if h + w > 2 {
        que.push_back((h - 1, w - 1, h + w - 3));
    }
    while let Some((x, y, i)) = que.pop_front() {
        for dir in 0..2 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if dist[nx][ny] == INF && i > 0 {
                que.push_back((nx, ny, i - 1));
            }

            dist[nx][ny] = dist[nx][ny].min((dist[x][y] + p[i] - a[nx][ny]).max(0));
        }
    }

    println!("{}", dist[0][0]);
}
