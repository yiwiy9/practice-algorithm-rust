use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    let ans = doubling(&a, 0, k);
    println!("{}", ans + 1);
}

pub fn doubling(next_nodes: &Vec<usize>, node: usize, k: usize) -> usize {
    let n = next_nodes.len();
    let max_log = (k as f64).log2().ceil() as usize;
    let mut doubling_table = vec![vec![0; n]; max_log + 1];
    doubling_table[0][..n].copy_from_slice(&next_nodes[..n]);
    for i in 1..=max_log {
        for j in 0..n {
            doubling_table[i][j] = doubling_table[i - 1][doubling_table[i - 1][j]];
        }
    }
    let mut current_node = node;
    for (i, i_steps_nodes) in doubling_table.iter().enumerate().take(max_log + 1) {
        if (k >> i) & 1 == 1 {
            current_node = i_steps_nodes[current_node];
        }
    }
    current_node
}

// fn main() {
//     input! {
//         n: usize,
//         mut k: i64,
//         a: [Usize1; n],
//     }

//     let loop_len;
//     let mut dist = vec![-1; n];
//     let mut cur_dist = 0;
//     let mut cur_pos = 0;
//     loop {
//         if dist[cur_pos] != -1 {
//             loop_len = cur_dist - dist[cur_pos];
//             break;
//         }
//         dist[cur_pos] = cur_dist;
//         cur_pos = a[cur_pos];
//         cur_dist += 1;
//     }

//     if k > dist[cur_pos] {
//         k = dist[cur_pos] + (k - dist[cur_pos]) % loop_len;
//     }

//     let mut ans = 0;
//     for _ in 0..k {
//         ans = a[ans];
//     }
//     println!("{}", ans + 1);
// }
