use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t],
    }

    for &(n, k) in &nk {
        // 桁DPのにおいがする
        // dp[i][smaller][j] := Xの2進数表記のi桁目まで決めて、
        //                      nより小さいことが確定しているかどうか（smaller）が決まってきて、
        //                      popCountの値が j である
        //                      ときのXの総和のMOD （桁ごとに和を計算できるから、桁DPが使える）
        // => sum_dp, cnt_dp を用意する
        // => new_sum = sum × 2 + digit × cnt (xを左にシフトさせて、末尾に1を追加)

        let n_chars = decimal_to_chars(n, 2);

        let mut cnt_dp = vec![vec![Mint::new(0); k + 1]; 2];
        let mut sum_dp = vec![vec![Mint::new(0); k + 1]; 2];

        // まだ何も決めていない時点で、popcount=0 の数は 1 個（X=0 の「空のプレフィックス」）
        cnt_dp[0][0] = Mint::new(1);

        for &n_i in &n_chars {
            let mut next_cnt_dp = vec![vec![Mint::new(0); k + 1]; 2];
            let mut next_sum_dp = vec![vec![Mint::new(0); k + 1]; 2];

            let n_digit = n_i as usize - '0' as usize;

            for smaller in 0..2 {
                for j in 0..=k {
                    for d in 0..=1 {
                        if smaller == 0 && d > n_digit {
                            // N より大きい場合はスキップ
                            break;
                        }

                        let nj = j + d;
                        if nj > k {
                            // 桁和が s を超える場合はスキップ
                            break;
                        }

                        let next_smaller = if smaller == 1 || d < n_digit { 1 } else { 0 };
                        next_cnt_dp[next_smaller][nj] += cnt_dp[smaller][j];
                        next_sum_dp[next_smaller][nj] +=
                            sum_dp[smaller][j] * 2 + Mint::new(d) * cnt_dp[smaller][j];
                    }
                }
            }

            cnt_dp = next_cnt_dp;
            sum_dp = next_sum_dp;
        }

        let ans = sum_dp[0][k] + sum_dp[1][k];
        println!("{}", ans.val());
    }
}

pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}
