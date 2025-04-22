use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            a_i: [usize; k],
        }
        a.push(a_i);
    }

    input! {
        b: [usize; n],
    }

    let mut b_map = vec![0; n + 1];
    for i in 0..n {
        b_map[b[i]] = i;
    }

    let mut ans = vec![0; n];
    for a_i in &a {
        let mut max_b_i = 0;
        for &a_j in a_i {
            max_b_i = max_b_i.max(b_map[a_j]);
        }
        ans[max_b_i] += 1;
    }

    for i in 1..n {
        ans[i] += ans[i - 1];
    }

    println!("{}", ans.iter().join("\n"));
}
