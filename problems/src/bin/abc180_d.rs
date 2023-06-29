use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    }

    if y <= x * a && y <= x + b {
        println!("{}", 0);
        return;
    }

    let m = b / x / (a - 1);
    let mut ans = 0;
    let mut n = 1;
    while n < m {
        n *= a;
        ans += 1;
    }

    let z = y - (x * n);
    ans += z / b;
    if z % b == 0 {
        ans -= 1;
    }

    println!("{}", ans);
}
