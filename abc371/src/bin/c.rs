use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m_g: usize,
        uv: [(Usize1,Usize1); m_g],
        m_h: usize,
        ab: [(Usize1,Usize1); m_h],
    }

    let mut cost = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                c: usize,
            }
            cost[i][j] = c;
        }
    }

    let mut graph_g = vec![vec![]; n];
    for &(u, v) in &uv {
        graph_g[u].push(v);
        graph_g[v].push(u);
    }

    let mut graph_h = vec![vec![]; n];
    for &(a, b) in &ab {
        graph_h[a].push(b);
        graph_h[b].push(a);
    }

    let mut g_index: Vec<usize> = (0..n).collect();

    let mut ans = 1 << 60;
    // next_permutation()
    permutohedron::heap_recursive(&mut g_index, |g_index| {
        let mut cur = 0;
        for i in 0..n {
            for j in i + 1..n {
                let in_g = graph_g[g_index[i]].contains(&g_index[j]);
                let in_h = graph_h[i].contains(&j);
                if in_g != in_h {
                    cur += cost[i][j];
                }
            }
        }
        ans = ans.min(cur);
    });

    println!("{}", ans);
}
