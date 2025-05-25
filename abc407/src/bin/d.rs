use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut sum = 0;
    for i in 0..h {
        for j in 0..w {
            sum ^= a[i][j];
        }
    }
    let mut max_val = sum;
    dfs(&a, h, w, sum, &mut HashSet::new(), &mut max_val, 0, 0);

    println!("{}", max_val);
}

pub fn dfs(
    a: &Vec<Vec<usize>>,
    h: usize,
    w: usize,
    sum: usize,
    used: &mut HashSet<(usize, usize)>,
    max_val: &mut usize,
    cur: usize,
    k: usize,
) {
    for kk in k..h * w {
        let ii = kk / w;
        let jj = kk % w;

        if used.contains(&(ii, jj)) {
            continue;
        }

        if !used.contains(&(ii + 1, jj)) && ii + 1 < h {
            let next = cur ^ a[ii][jj] ^ a[ii + 1][jj];
            *max_val = std::cmp::max(*max_val, sum ^ next);
            used.insert((ii, jj));
            used.insert((ii + 1, jj));
            dfs(a, h, w, sum, used, max_val, next, kk + 1);
            used.remove(&(ii, jj));
            used.remove(&(ii + 1, jj));
        }

        if !used.contains(&(ii, jj + 1)) && jj + 1 < w {
            let next = cur ^ a[ii][jj] ^ a[ii][jj + 1];
            *max_val = std::cmp::max(*max_val, sum ^ next);
            used.insert((ii, jj));
            used.insert((ii, jj + 1));
            dfs(a, h, w, sum, used, max_val, next, kk + 1);
            used.remove(&(ii, jj));
            used.remove(&(ii, jj + 1));
        }
    }
}
