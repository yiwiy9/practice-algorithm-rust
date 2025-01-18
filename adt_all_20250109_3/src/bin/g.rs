use proconio::input;

fn main() {
    input! {
        n: usize,
        s: (i64,i64),
        t: (i64,i64),
        xyr: [(i64,i64,i64); n],
    }

    let mut s_i = n;
    let mut t_i = n;
    let mut graph = vec![vec![]; n];
    for (i, &(x, y, r)) in xyr.iter().enumerate() {
        if (x - s.0).pow(2) + (y - s.1).pow(2) == r.pow(2) {
            s_i = i;
        }
        if (x - t.0).pow(2) + (y - t.1).pow(2) == r.pow(2) {
            t_i = i;
        }
        for (j, &(x_j, y_j, r_j)) in xyr.iter().enumerate() {
            let dist = (x - x_j).pow(2) + (y - y_j).pow(2);

            // どちらか一方の円が他方の円の内部にあるケース
            if dist < (r - r_j).pow(2) {
                continue;
            }
            // どちらの円も他方の円の外部にあるケース
            if dist > (r + r_j).pow(2) {
                continue;
            }

            graph[i].push(j);
        }
    }

    let mut seen = vec![false; n];
    dfs(&graph, &mut seen, s_i);

    println!("{}", if seen[t_i] { "Yes" } else { "No" });
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, next_v);
    }
}
