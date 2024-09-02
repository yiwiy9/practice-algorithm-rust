use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // sのi文字目まで見て、sの後ろからk文字目がjであり、そのjが回文でない場合の数
    let mut dp = vec![vec![0; 1 << k]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..(1 << k) {
            if dp[i][j] == 0 {
                continue;
            }

            let new_j_base = j << 1 & ((1 << k) - 1);

            match s[i] {
                '?' => {
                    for l in 0..2 {
                        let new_j = new_j_base | l;
                        if i < k - 1 || !is_palindrome(new_j, k) {
                            dp[i + 1][new_j] += dp[i][j];
                            dp[i + 1][new_j] %= MOD;
                        }
                    }
                }
                _ => {
                    let new_j = new_j_base | (s[i] as usize - 'A' as usize);
                    if i < k - 1 || !is_palindrome(new_j, k) {
                        dp[i + 1][new_j] += dp[i][j];
                        dp[i + 1][new_j] %= MOD;
                    }
                }
            }
        }
    }

    println!("{}", dp[n].iter().fold(0, |acc, x| (acc + x) % MOD));
}

fn is_palindrome(bit: usize, k: usize) -> bool {
    let mut bit = bit;
    let mut s = vec![];
    for _ in 0..k {
        s.push(bit % 2);
        bit /= 2;
    }
    let mut l = 0;
    let mut r = k - 1;
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
