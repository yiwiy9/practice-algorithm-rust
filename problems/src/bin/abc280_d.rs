use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let factors = prime_factors(k);

    let mut ans = 1;
    for &(num, cnt) in &factors {
        let mut n = 0;
        let mut used_num = cnt as i32;
        while used_num > 0 {
            n += num;
            let mut x = n;
            while x % num == 0 {
                x /= num;
                used_num -= 1;
            }
        }
        ans = ans.max(n);
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
