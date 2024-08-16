use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

/**
 * https://atcoder.jp/contests/abc215/tasks/abc215_d
 * https://atcoder.jp/contests/abc215/editorial/2482
 *
 * AのLCMとって、それに対してGCDが1のものを求めるのはダメ！！（前も同じことをやっている）
 * => オーバーフローする可能性がある (10^5を10^5回かける)
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut factors = HashSet::new();
    for &a_i in &a {
        prime_factors(a_i, &mut factors);
    }

    let mut coprimes = vec![true; m + 1];
    for factor in factors {
        for i in (1..).map(|x| factor * x).take_while(|&x| x <= m) {
            coprimes[i] = false;
        }
    }

    let ans: Vec<usize> = (1..=m).filter(|&x| coprimes[x]).collect();

    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
}

pub fn prime_factors(mut n: usize, factors: &mut HashSet<usize>) {
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i != 0 {
            continue;
        }
        while n % i == 0 {
            n /= i;
        }
        factors.insert(i);
    }
    if n != 1 {
        factors.insert(n);
    }
}
