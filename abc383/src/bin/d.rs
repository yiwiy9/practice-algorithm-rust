use num_integer::Roots;
use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/abc383/tasks/abc383_d
/// https://atcoder.jp/contests/abc383/editorial/11508
///
/// 約数の個数
/// https://www.try-it.jp/chapters-4962/sections-4963/lessons-5000/
/// 素因数分解した時の場合の数を考える
/// 9 = (x^0, x^1, x^2) * (y^0, y^1, y^2) OR (x^0, ... , x^8)
fn main() {
    input! {
        n: usize,
    }

    let primes = eratosthenes_sieve(n.sqrt());
    let prime_squares = primes.iter().map(|&x| x * x).collect::<Vec<_>>();

    let mut ans = 0;
    for &sq in &prime_squares {
        if sq * sq * sq * sq > n {
            break;
        }
        ans += 1;
    }

    for (i, &sq) in prime_squares.iter().enumerate() {
        let cnt = prime_squares.upper_bound(&(n / sq));
        if cnt <= i + 1 {
            break;
        }
        ans += cnt - (i + 1);
    }

    println!("{}", ans);
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
