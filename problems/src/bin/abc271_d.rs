use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![String::from("N"); s + 1]; n + 1];
    dp[0][0] = String::from("");
    for i in 0..n {
        for j in 0..s {
            if dp[i][j] == "N" {
                continue;
            }
            if j + ab[i].0 <= s {
                dp[i + 1][j + ab[i].0] = dp[i][j].clone() + "H"
            }
            if j + ab[i].1 <= s {
                dp[i + 1][j + ab[i].1] = dp[i][j].clone() + "T"
            }
        }
    }

    println!(
        "{}",
        if dp[n][s] != "N" {
            String::from("Yes\n") + dp[n][s].as_str()
        } else {
            String::from("No")
        }
    );
}
