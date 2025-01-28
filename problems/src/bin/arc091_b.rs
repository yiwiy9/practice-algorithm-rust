use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k == 0 {
        println!("{}", n * n);
        return;
    }

    let mut ans = 0;
    for b in k + 1..=n {
        let p = n / b;
        let r = n % b;
        ans += p * (b - k);
        if r >= k {
            ans += r - k + 1;
        }
    }

    println!("{}", ans);
}
