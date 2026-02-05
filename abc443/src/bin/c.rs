use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut ans: usize = 0;
    let mut open = 0;
    for &a_i in &a {
        if a_i < open {
            continue;
        }
        ans += a_i - open;
        open = a_i + 100;
    }

    if open < t {
        ans += t - open;
    }

    println!("{}", ans);
}
