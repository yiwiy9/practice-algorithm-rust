use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut cnt = vec![vec![0; 26]; n + 1];
    for i in 0..n {
        for j in 0..26 {
            cnt[i + 1][j] += cnt[i][j]
                + if (s[i] as usize - 'a' as usize) == j {
                    1
                } else {
                    0
                };
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let j: usize = s[i] as usize - 'a' as usize;
        let right = (i + r + 1).min(n);
        let left = (i + l).min(n);
        ans += cnt[right][j] - cnt[left][j];
    }

    println!("{}", ans);
}
