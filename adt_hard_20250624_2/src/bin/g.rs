use num_integer::Roots;
use proconio::input;
const MAX_N: usize = 1_000_001;

fn main() {
    input! {
        n: usize,
    }

    let (is_prime, primes) = eratosthenes_sieve(MAX_N);

    let mut prefix_sums = vec![0; MAX_N];
    for num in 1..MAX_N {
        prefix_sums[num] = prefix_sums[num - 1];
        if is_prime[num] {
            prefix_sums[num] += 1;
        }
    }

    let mut ans = 0;
    for (i, &a) in primes.iter().enumerate() {
        if a * a * a * a * a >= n {
            break;
        }

        for j in i + 1..primes.len() {
            let b = primes[j];
            if a * a * b * b * b >= n {
                break;
            }

            let c_2 = n / (a * a * b);

            ans += prefix_sums[c_2.sqrt()] - prefix_sums[b];
        }
    }

    println!("{}", ans);
}

pub fn eratosthenes_sieve(n: usize) -> (Vec<bool>, Vec<usize>) {
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
    (is_prime, primes)
}
