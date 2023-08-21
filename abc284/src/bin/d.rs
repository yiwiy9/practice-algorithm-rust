use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let primes = eratosthenes_sieve(3_000_000);
    for _ in 0..t {
        input! {
            mut n: usize,
        }
        for prime in &primes.0 {
            if n % (prime * prime) == 0 {
                println!("{} {}", prime, n / (prime * prime));
                break;
            }
            if n % prime == 0 {
                println!("{} {}", (n / prime).sqrt(), prime);
                break;
            }
        }
    }
}

pub fn eratosthenes_sieve(n: usize) -> (Vec<usize>, Vec<bool>) {
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
    (primes, is_prime)
}

pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
