use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let k_divisors = divisors(k);
    let mut ans = 0;
    for (i, c) in k_divisors.iter().rev().enumerate() {
        for a in k_divisors.iter().take(i + 1) {
            if a * c > k {
                break;
            }
            if k % (a * c) != 0 {
                continue;
            }
            let b = k / (a * c);
            if *a <= b && b <= *c {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

pub fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort();
    divisors
}
