use num::integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let edge = gcd(gcd(a, b), c);
    let ans = (a + b + c) / edge - 3;

    println!("{}", ans);
}
