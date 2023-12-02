use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let a_prime_factors = prime_factors(a);
    let b_prime_factors = prime_factors(b);

    let mut a_primes = BTreeSet::new();
    for (a_prime, _) in &a_prime_factors {
        a_primes.insert(*a_prime);
    }

    let mut ans = 1;
    for (b_prime, _) in &b_prime_factors {
        if a_primes.contains(b_prime) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

pub fn prime_factors(mut n: usize) -> Vec<(usize, usize)> {
    let mut factors: Vec<(usize, usize)> = Vec::new();
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i != 0 {
            continue;
        }
        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        factors.push((i, count));
    }
    if n != 1 {
        factors.push((n, 1));
    }
    factors
}
