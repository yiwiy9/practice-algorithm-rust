use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        p: Usize1,
        q: Usize1,
        s: [[usize; w]; h],
    }

    println!("{}", dijkstra(&s, (p, q), x));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cost: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(field: &Vec<Vec<usize>>, s: (usize, usize), num_x: usize) -> usize {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let h = field.len();
    let w = field[0].len();

    let mut seen = vec![vec![false; w]; h];
    let mut pq = std::collections::BinaryHeap::new();

    seen[s.0][s.1] = true;
    pq.push(Node {
        x: s.0,
        y: s.1,
        cost: 0,
    });
    let mut cur_cost = field[s.0][s.1];

    while let Some(Node { x, y, cost }) = pq.pop() {
        // 1/X 未満 なら吸収 => X/X は吸収しない、X+1/X は吸収
        if cost >= (cur_cost + num_x - 1) / num_x {
            break;
        }

        cur_cost += cost;

        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if seen[nx][ny] {
                continue;
            }

            seen[nx][ny] = true;
            pq.push(Node {
                x: nx,
                y: ny,
                cost: field[nx][ny],
            });
        }
    }
    cur_cost
}
