use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let prime_factors = prime_factors(n);

    println!(
        "{}",
        prime_factors
            .iter()
            .map(|(_, cnt)| {
                let mut mut_cnt = *cnt;
                let mut num = 1;
                let mut sum = 0;
                while mut_cnt >= num {
                    sum += 1;
                    mut_cnt -= num;
                    num += 1;
                }
                sum
            })
            .sum::<usize>()
    );
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
