use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }

    let mut ans = t - x * d;
    for i in 0..m {
        if i < x {
            ans += d;
        }
    }

    println!("{}", ans);
}
