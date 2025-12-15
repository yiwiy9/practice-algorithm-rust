use proconio::input;
use superslice::*;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
    }

    p.push(0);

    let mut p_2 = vec![];
    for &p_i in &p {
        for &p_j in &p {
            p_2.push(p_i + p_j);
        }
    }

    p_2.sort();

    let mut ans = 0;
    for &p_i in &p_2 {
        if p_i > m {
            break;
        }

        let target = m - p_i;
        let upper_i = p_2.upper_bound(&target); // p_2 include 0

        ans = ans.max(p_i + p_2[upper_i - 1]);
    }

    println!("{}", ans);
}
