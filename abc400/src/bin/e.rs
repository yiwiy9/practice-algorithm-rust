use num::integer::gcd;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        q: usize,
        queries: [usize; q],
    }

    let primes = eratosthenes_sieve(1_000_000);
    let mut prime_squares = vec![];
    for &p in &primes {
        let p2 = p * p;
        let mut cur = p2;
        while cur <= 1_000_000_000_000 {
            prime_squares.push(cur);

            if cur > 1_000_000_000_000 / p2 + 1 {
                break;
            }
            cur *= p2;
        }
    }
    prime_squares.sort();

    let mut numbers = vec![];
    for i in 0..prime_squares.len() {
        for j in i + 1..prime_squares.len() {
            if prime_squares[i] > 1_000_000_000_000 / prime_squares[j] + 1 {
                break;
            }
            if gcd(prime_squares[i], prime_squares[j]) != 1 {
                continue;
            }
            numbers.push(prime_squares[i] * prime_squares[j]);
        }
    }
    numbers.sort();

    for &a in &queries {
        let upper_bound = numbers.upper_bound(&a);
        println!("{}", numbers[upper_bound - 1]);
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
