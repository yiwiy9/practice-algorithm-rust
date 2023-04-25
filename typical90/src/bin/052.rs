use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut ans = 1;

    for dice in a {
        let sum: usize = dice.iter().sum::<usize>() % MOD;
        ans *= sum;
        ans %= MOD;
    }

    println!("{}", ans);
}

// fn main() {
//     input! {
//         n: usize,
//         a: [[usize; 6]; n],
//     }

//     let mut dp = vec![vec![0; 6]; n];

//     dp[0] = a[0].clone();

//     for i in 1..n {
//         let before_sum: usize = dp[i - 1].iter().sum::<usize>() % MOD;
//         for j in 0..6 {
//             dp[i][j] = a[i][j] * before_sum % MOD;
//         }
//     }

//     println!("{}", dp[n - 1].iter().sum::<usize>() % MOD);
// }
