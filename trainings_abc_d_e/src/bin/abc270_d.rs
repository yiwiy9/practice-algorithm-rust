use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    // 山の残りの石の数がi個のときにの "先手の" 可能な最大得点数
    let mut dp = vec![-1; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for &a_i in &a {
            if i < a_i || dp[i - a_i] == -1 {
                continue;
            }

            // それぞれが最善を尽くすので、相手のターンの影響を受ける必要がある
            let before = (i - a_i) as i64 - dp[i - a_i];
            dp[i] = dp[i].max(before + a_i as i64);
        }
    }

    println!("{}", dp[n]);
}
