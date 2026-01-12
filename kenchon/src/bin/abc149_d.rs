use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    }

    let mut ans: usize = 0;
    for j in 0..k {
        let mut dp = vec![0; 3];

        let mut i = j;
        while i < n {
            let mut next_dp = vec![0; 3];

            for l in 0..3 {
                next_dp[l] = dp[(l + 1) % 3].max(dp[(l + 2) % 3]);
                match t[i] {
                    'r' => {
                        if l == 2 {
                            next_dp[l] += p
                        }
                    }
                    's' => {
                        if l == 0 {
                            next_dp[l] += r
                        }
                    }
                    'p' => {
                        if l == 1 {
                            next_dp[l] += s
                        }
                    }
                    _ => unreachable!(),
                }
            }

            dp = next_dp;
            i += k;
        }

        ans += dp.iter().max().unwrap();
    }

    println!("{}", ans);
}
