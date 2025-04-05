use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = 1;
    let mut cur = 1;
    for _ in 1..=m {
        cur *= n;
        ans += cur;
        if ans > 1_000_000_000 {
            println!("inf");
            return;
        }
    }

    println!("{}", ans);
}
