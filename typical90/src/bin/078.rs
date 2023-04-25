use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut count = 0;
    for (v, next_vs) in g.iter().enumerate() {
        let mut min_count = 0;
        for &next_v in next_vs {
            if next_v < v {
                min_count += 1;
            }
        }
        if min_count == 1 {
            count += 1;
        }
    }

    println!("{}", count);
}
