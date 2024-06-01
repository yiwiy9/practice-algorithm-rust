use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for a_i in a {
        if a_i < p {
            ans += 1;
        }
    }

    println!("{}", ans);
}
