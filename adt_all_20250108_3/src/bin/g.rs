use proconio::input;

/// https://atcoder.jp/contests/adt_all_20250108_3/tasks/abc284_d
/// https://atcoder.jp/contests/adt_all_20250108_3/editorial/5467
/// 適切に計算量を見積り、全探索を行う
fn main() {
    input! {
        t: usize,
        tests: [usize; t],
    }

    let primes = eratosthenes_sieve(3000001);
    // let prime_set = primes.iter().collect::<std::collections::BTreeSet<_>>();

    for &n in &tests {
        for &p in &primes {
            // 必ずしも、p <= sqrt3(n) とは限らない。反例: q = 2 の場合、p*p ~ n となる
            // if n % (p * p) == 0 && prime_set.contains(&(n / (p * p))) {
            //     println!("{} {}", p, n / (p * p));
            //     break;
            // }

            // min(p, q) <= sqrt3(n) ということは確実に言えるので、qの可能性も考える
            if n % p == 0 {
                if n % (p * p) == 0 {
                    println!("{} {}", p, n / (p * p));
                } else {
                    println!("{} {}", ((n / p) as f64).sqrt() as usize, p);
                }
                break; // n の条件より、素因数が見つかれば必ず答えが一意に定まる
            }
        }
    }
}

pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
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
