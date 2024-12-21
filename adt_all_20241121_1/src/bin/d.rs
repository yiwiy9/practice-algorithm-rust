use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
    }

    let mut ans = 0;
    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
