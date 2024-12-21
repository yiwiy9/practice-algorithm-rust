use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let primes = eratosthenes_sieve(201);
    let prime_set = primes
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();

    for i in a..=b {
        let mut can_be_prime = false;
        for j in c..=d {
            if prime_set.contains(&(i + j)) {
                can_be_prime = true;
                break;
            }
        }
        if !can_be_prime {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
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
