use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, usize, usize); q],
    }

    println!("{}", dfs_homogeneous(n, m, &abcd, &mut vec![0; n], 0, 1));
}

// 重複組合せ: mHn = m+n-1Cn
fn dfs_homogeneous(
    n: usize,
    m: usize,
    abcd: &Vec<(usize, usize, usize, usize)>,
    a: &mut Vec<usize>,
    idx: usize,
    start_num: usize,
) -> usize {
    if idx == n {
        let mut cur = 0;
        for &(a_i, b_i, c_i, d_i) in abcd {
            if a[b_i] - a[a_i] == c_i {
                cur += d_i;
            }
        }
        return cur;
    }

    let mut ans = 0;
    for num in start_num..=m {
        a[idx] = num;
        let cur = dfs_homogeneous(n, m, abcd, a, idx + 1, num);
        ans = ans.max(cur);
    }
    ans
}
