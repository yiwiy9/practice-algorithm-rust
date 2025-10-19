use proconio::input;

fn main() {
    input! {
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }

    let mut ans = 0;
    for i in 0..x {
        if i % (a + b) < a {
            ans += s;
        }
    }

    println!("{}", ans);
}
