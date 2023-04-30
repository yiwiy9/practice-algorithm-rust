use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let c_upper = (n as f64 / 12.0).sqrt() as usize;
    let primes = eratosthenes_sieve(c_upper);

    let mut cnt = 0;
    for (c_i, c) in primes.iter().enumerate().rev() {
        for (a_i, a) in primes.iter().enumerate() {
            if a * a * c * c > n {
                break;
            }

            for b_i in (a_i + 1..c_i).rev() {
                if a * a * primes[b_i] * c * c <= n {
                    cnt += b_i - a_i;
                    break;
                }
            }
        }
    }

    println!("{}", cnt);
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
