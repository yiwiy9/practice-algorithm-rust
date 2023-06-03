use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        p: [(i32,i32); n],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(x_i, y_i)) in p.iter().enumerate() {
        for (j, &(x_j, y_j)) in p.iter().enumerate() {
            if (x_i - x_j) * (x_i - x_j) + (y_i - y_j) * (y_i - y_j) <= d * d {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let dist = bfs(&graph, 0);

    for &d in &dist {
        if d != 1 << 30 {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dist[s] = 0;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    dist
}
