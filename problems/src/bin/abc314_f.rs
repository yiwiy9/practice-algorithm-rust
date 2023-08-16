use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        pq: [(Usize1,Usize1); n-1],
    }

    let mut uf = UnionFind::new(n);
    let mut graph = vec![vec![]; 2 * (n - 1) + 1];
    let mut nodes_len = vec![1; 2 * (n - 1) + 1];
    let mut root_nodes = (0..n).collect::<Vec<_>>();
    let mut cur_node = n;

    for &(p, q) in &pq {
        let p_root = uf.find(p);
        let q_root = uf.find(q);

        graph[cur_node].push(root_nodes[p_root]);
        graph[cur_node].push(root_nodes[q_root]);
        nodes_len[cur_node] = nodes_len[root_nodes[p_root]] + nodes_len[root_nodes[q_root]];

        uf.union(p_root, q_root);
        root_nodes[uf.find(p_root)] = cur_node;

        cur_node += 1;
    }

    let mut expected = vec![0; 2 * (n - 1) + 1];
    dfs(&graph, &nodes_len, &mut expected, cur_node - 1);

    println!("{}", expected.iter().take(n).join(" "));
}

pub fn dfs(graph: &Vec<Vec<usize>>, nodes_len: &Vec<usize>, expected: &mut Vec<usize>, v: usize) {
    for &next_v in &graph[v] {
        expected[next_v] = expected[v] + (nodes_len[next_v] * mod_inv(nodes_len[v], MOD) % MOD);
        expected[next_v] %= MOD;
        dfs(graph, nodes_len, expected, next_v);
    }
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
