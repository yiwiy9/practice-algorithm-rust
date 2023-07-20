use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut k: i64,
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

    if k > dist[cur_pos] {
        k = dist[cur_pos] + (k - dist[cur_pos]) % loop_len;
    }

    let mut ans = 0;
    for _ in 0..k {
        ans = a[ans];
    }
    println!("{}", ans + 1);
}
