use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut ans = n;
    for bit in 0..(1 << n) {
        let mut cnt = 0;
        let mut t = vec![false; m];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                cnt += 1;
                for j in 0..m {
                    if s[i][j] == 'o' {
                        t[j] = true;
                    }
                }
            }
        }

        if t.iter().all(|&x| x) {
            ans = ans.min(cnt);
        }
    }

    println!("{}", ans);
}
