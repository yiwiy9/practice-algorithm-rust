use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let factors = prime_factors(n);
    let prime_count: usize = factors.iter().map(|(_, exp)| exp).sum();

    let mut ans = 0;
    let mut i = 1;
    while i < prime_count {
        i *= 2;
        ans += 1;
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
