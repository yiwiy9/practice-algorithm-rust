use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
    }

    let mut compressed = vec![vec![0; m]; l];
    for (i, &a_i) in a.iter().enumerate() {
        let j = i % l;
        for k in 0..m {
            compressed[j][k] += (m + k - a_i) % m;
        }
    }

    let mut dp = compressed.first().unwrap().clone();
    for mod_comp in compressed.iter().skip(1) {
        let mut next_dp = vec![INF; m];
        for j in 0..m {
            for (k, &val) in mod_comp.iter().enumerate() {
                let next_j = (j + k) % m;
                next_dp[next_j] = next_dp[next_j].min(dp[j] + val);
            }
        }
        dp = next_dp;
    }

    println!("{}", dp[0]);
}
