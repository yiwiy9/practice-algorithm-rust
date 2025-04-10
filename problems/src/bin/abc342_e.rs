use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcab: [(usize, usize, usize, usize, Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(_, _, _, _, a, b)) in ldkcab.iter().enumerate() {
        graph[b].push((a, i));
    }

    let departure_time = dijkstra(&graph, &ldkcab);

    println!(
        "{}",
        departure_time
            .iter()
            .take(n - 1)
            .map(|&x| if x == 0 {
                "Unreachable".to_string()
            } else {
                x.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    time: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(
    graph: &[Vec<(usize, usize)>],
    ldkcab: &[(usize, usize, usize, usize, usize, usize)],
) -> Vec<usize> {
    let n = graph.len();
    let mut departure_time = vec![0; n];
    let mut pq = std::collections::BinaryHeap::new();

    for &(from, i) in &graph[n - 1] {
        let (l, d, k, _, _, _) = ldkcab[i];
        let last_departure_time = l + (k - 1) * d;
        departure_time[from] = last_departure_time;
        pq.push(Node {
            vertex: from,
            time: last_departure_time,
        });
    }

    while let Some(Node { vertex, time }) = pq.pop() {
        if departure_time[vertex] > time {
            continue;
        }
        for &(from, i) in &graph[vertex] {
            let (l, d, k, c, _, _) = ldkcab[i];
            if c > time {
                continue;
            }

            let deadline = time - c;
            if l > deadline {
                continue;
            }

            let last_train = (deadline - l) / d;
            let last_departure_time = l + last_train.min(k - 1) * d;

            if last_departure_time > departure_time[from] {
                departure_time[from] = last_departure_time;
                pq.push(Node {
                    vertex: from,
                    time: last_departure_time,
                });
            }
        }
    }
    departure_time
}
