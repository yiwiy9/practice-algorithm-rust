use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let m_divisors = divisors(m);
    for &div in &m_divisors {
        if div >= n {
            println!("{}", m / div);
            return;
        }
    }
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
