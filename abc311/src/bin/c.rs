use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let loop_len;
    let mut dist = vec![-1; n];
    let mut cur_dist = 0;
    let mut cur_pos = 0;
    loop {
        if dist[cur_pos] != -1 {
            loop_len = cur_dist - dist[cur_pos];
            break;
        }
        dist[cur_pos] = cur_dist;
        cur_pos = a[cur_pos];
        cur_dist += 1;
    }

    let mut ans = vec![];
    for _ in 0..loop_len {
        ans.push(cur_pos + 1);
        cur_pos = a[cur_pos];
    }

    println!("{}", loop_len);
    println!("{}", ans.iter().join(" "))
}
