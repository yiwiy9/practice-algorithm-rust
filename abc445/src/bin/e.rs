use ac_library::modint::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

const MAX_NUM: usize = 10_000_010;

/// https://atcoder.jp/contests/abc445/tasks/abc445_e
/// https://atcoder.jp/contests/abc445/editorial/15897
///
/// # メモ（なぜセグ木で gcd を取る方針がダメか / この問題の本質）
///
/// ## 1. まず「LCM は gcd では作れない」
/// セグ木で `gcd(除外区間)` を取って何か補正すれば LCM が作れそうに見えるが、根本的に情報が足りない。
///
/// - 素数 `p` の指数（素因数分解の回数）で見ると:
///   - `gcd` は指数の **min**
///   - `lcm` は指数の **max**
///   - `積` は指数の **sum**
///
/// 2個のときだけは `max = sum - min` で復元できる（`lcm(x,y)=x*y/gcd(x,y)`）が、
/// 3個以上になると **max を min と sum だけから復元できない**。
/// （「最大がどれか」「最大が何回出たか」「2位はいくつか」などの情報が消えている）
///
/// 反例: A=[2,4,8]
/// - 除外後 LCM は lcm(4,8)=8
/// - でも「積」や「gcd(除外)」を使った補正は 8 にならず破綻する。
///
/// → よって「gcd セグ木 + 積（mod）で LCM を再現」は方向がズレている。
///
/// ## 2. この問題で必要なのは「各素数の 1位と2位」
/// LCM の定義より、任意の素数 `p` について:
/// - `lcm(...)` に含まれる `p` の指数は、各要素の `p` 指数の **最大値**。
///
/// さらに、`A_i` を除いた LCM `L_i` は:
/// - `A_i` の `p` 指数が最大未満 → `L_i` の指数は **最大値（1位）**
/// - `A_i` の `p` 指数が最大そのもの → `L_i` の指数は **2位（2番目の最大）**
///
/// つまり各 `p` ごとに以下を持てば、各 `i` の LCM を作れる。
/// - `e1[p]` = 最大指数
/// - `e2[p]` = 2番目の最大指数
///
/// ## 3. 解法の流れ（概要）
/// 1) 全要素を素因数分解して、各素数 `p` の指数 `exp[i][p]` を得る
/// 2) 各 `p` について `e1[p]` と `e2[p]`（最大・2位）を集計する
/// 3) 全体 LCM `L = ∏ p^{e1[p]}` を mod で構築する
/// 4) 各 `i` の答えは
///    - `L` から「`A_i` が最大を担っていた素数」だけ `p^{e1-e2}` 分を割る（=指数を 2位に落とす）
///
/// セグ木を使うなら「gcd」ではなく「max/second max 情報（=e1,e2）」を保持できる構造が必要だが、
/// この問題は素因数分解で直接 `e1,e2` を集計するのが最短。
fn main() {
    input! {
        t: usize,
    }

    let primes = eratosthenes_sieve(MAX_NUM);

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }

        let mut first_prime_map = HashMap::new();
        let mut second_prime_map = HashMap::new();
        for &a_i in &a {
            let a_primes_cnt = factorize_with_primes(a_i, &primes);
            for &(p, cnt) in &a_primes_cnt {
                let first = first_prime_map.entry(p).or_default();
                let second = second_prime_map.entry(p).or_default();
                if cnt > *first {
                    *second = *first;
                    *first = cnt;
                } else if cnt > *second {
                    *second = cnt;
                }
            }
        }

        let lcm = first_prime_map
            .iter()
            .fold(Mint::new(1), |acc, (&p, &cnt)| {
                acc * Mint::new(p).pow(cnt as u64)
            });

        let mut ans = vec![lcm; n];
        for (i, &a_i) in a.iter().enumerate() {
            let a_primes_cnt = factorize_with_primes(a_i, &primes);
            for &(p, cnt) in &a_primes_cnt {
                let first = *first_prime_map.get(&p).unwrap_or(&0);
                let second = *second_prime_map.get(&p).unwrap_or(&0);
                if cnt == first {
                    ans[i] /= Mint::new(p).pow((first - second) as u64);
                }
            }
        }

        println!("{}", ans.iter().join(" "));
    }
}

/// エラトステネスのふるい
/// `1..=n` の素数を列挙して返す。
/// ## 計算量
/// - `O(n log log n)`（典型）
pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut primes = vec![];
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

/// 素因数分解（素数リスト版）
/// `primes`（昇順の素数列）でのみ試し割りする。
/// `p*p > n` で打ち切り、最後に `n!=1` ならそれが残りの素因数。
/// 典型: `primes = eratosthenes_sieve(√maxA)` を1回作って使い回す。
/// ## 計算量
/// - 目安: `O(π(√n) + ω(n))`
///   - `π(x)` は **x 以下の素数の個数**（prime counting function）
///   - `ω(n)` は **n の異なる素因数の個数**
pub fn factorize_with_primes(mut n: usize, primes: &[usize]) -> Vec<(usize, usize)> {
    debug_assert!(n >= 1);
    let mut factors: Vec<(usize, usize)> = Vec::new();
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p != 0 {
            continue;
        }
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        factors.push((p, count));
    }
    if n != 1 {
        factors.push((n, 1));
    }
    factors
}
