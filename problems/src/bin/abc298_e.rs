use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut dp = vec![vec![Mint::new(0); n + 1]; n + 1];
    for j in 1..=n {
        dp[n][j] = Mint::new(1);
    }

    for i in (1..n).rev() {
        for j in (1..n).rev() {
            let mut p_sum = Mint::new(0);
            for p_num in 1..=p {
                if p_num + i >= n {
                    p_sum += 1;
                    continue;
                }

                let mut q_sum = Mint::new(0);
                for q_num in 1..=q {
                    if q_num + j >= n {
                        continue;
                    }
                    q_sum += dp[p_num + i][q_num + j];
                }
                p_sum += q_sum / q;
            }
            dp[i][j] = p_sum / p;
        }
    }

    println!("{}", dp[a][b]);
}
