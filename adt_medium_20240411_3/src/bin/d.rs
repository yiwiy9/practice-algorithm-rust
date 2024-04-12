use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut has_edges = vec![vec![false; n]; n];
    for (u, v) in uv {
        has_edges[u][v] = true;
        has_edges[v][u] = true;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if has_edges[i][j] && has_edges[j][k] && has_edges[k][i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
