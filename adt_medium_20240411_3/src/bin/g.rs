use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut d = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            d_row: [usize; n-i-1],
        }

        for (j, &d_ij) in d_row.iter().enumerate() {
            d[i][i + 1 + j] = d_ij;
        }
    }

    let mut dp = vec![0; 1 << n];
    for bit in 0..1 << n {
        let mut left = 0;
        for i in 0..n {
            if bit & 1 << i == 0 {
                left = i;
                break;
            }
        }
        for i in left + 1..n {
            if bit & 1 << i == 0 {
                let right = i;
                let new_bit = bit | 1 << left | 1 << right;
                dp[new_bit] = dp[new_bit].max(dp[bit] + d[left][right]);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
