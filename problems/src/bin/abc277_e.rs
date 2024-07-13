use proconio::{input, marker::Usize1};

const INF: usize = 1 << 30;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uva: [(Usize1,Usize1,usize); m],
        s: [Usize1; k],
    }

    let mut graph = vec![vec![vec![]; n]; 2];
    for (u, v, a) in uva {
        graph[a][u].push(v);
        graph[a][v].push(u);
    }

    let mut switches = vec![false; n];
    for &u in &s {
        switches[u] = true;
    }

    let dist = bfs(n, &graph, &switches);

    println!(
        "{}",
        if dist[0][n - 1] == INF && dist[1][n - 1] == INF {
            -1
        } else {
            dist[0][n - 1].min(dist[1][n - 1]) as i32
        }
    );
}

fn bfs(n: usize, graph: &Vec<Vec<Vec<usize>>>, s: &Vec<bool>) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![INF; n]; 2];
    let mut que = std::collections::VecDeque::new();

    dist[1][0] = 0;
    que.push_back((0, 1));

    while let Some((u, state)) = que.pop_front() {
        for &v in &graph[state][u] {
            if dist[state][v] != INF {
                continue;
            }
            dist[state][v] = dist[state][u] + 1;
            que.push_back((v, state));
        }

        if s[u] {
            let switched_state = state ^ 1;
            for &v in &graph[switched_state][u] {
                if dist[switched_state][v] != INF {
                    continue;
                }
                dist[switched_state][v] = dist[state][u] + 1;
                que.push_back((v, switched_state));
            }
        }
    }
    dist
}
