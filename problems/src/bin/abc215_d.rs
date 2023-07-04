use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut not_using_nums = vec![true; m + 1];
    for &a_i in &a {
        let a_primes = prime_factors(a_i);
        for &(a_prime, _) in a_primes.iter() {
            if a_prime > m {
                continue;
            }
            if not_using_nums[a_prime] {
                let mut a_prime_multiple = a_prime;
                loop {
                    if a_prime_multiple > m {
                        break;
                    }
                    not_using_nums[a_prime_multiple] = false;
                    a_prime_multiple += a_prime;
                }
            }
        }
    }

    let ans = (1..=m).filter(|&i| not_using_nums[i]).collect::<Vec<_>>();
    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
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
