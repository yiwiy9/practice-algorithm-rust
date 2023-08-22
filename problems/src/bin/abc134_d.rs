use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = vec![];
    let mut cnt = vec![0; n + 1];
    for (i, &a_i) in a.iter().enumerate().rev() {
        if a_i != cnt[i + 1] {
            ans.push(i + 1);
            let divisors = divisors(i + 1);
            for &d in &divisors {
                cnt[d] += 1;
                cnt[d] %= 2;
            }
        }
    }

    ans.sort();

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!("{}", ans.iter().join(" "));
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
