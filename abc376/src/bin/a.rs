use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize; n],
    }

    let mut ans = 1;
    let mut before = t[0];
    for i in 1..n {
        if t[i] - before >= c {
            ans += 1;
            before = t[i];
        }
    }

    println!("{}", ans);
}
