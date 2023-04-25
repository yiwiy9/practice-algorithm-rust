use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        println!("No");
        return;
    }
    let d = sum / 10;

    let mut s = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        s[i + 1] = s[i] + a[i % n];
    }

    let mut ok = false;
    for i in 0..n {
        let j = s.lower_bound(&(d + s[i]));
        if s[j] == d + s[i] {
            ok = true;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
