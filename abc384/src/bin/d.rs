use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let a_sum = a.iter().sum::<usize>();
    let remains_s = s % a_sum;

    let mut front_s = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        front_s[i + 1] = front_s[i] + a[i % n];
    }

    let mut ok = false;
    for i in 0..=n {
        let num = remains_s + front_s[i];
        let idx = front_s.lower_bound(&num);
        if front_s[idx] == num {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
