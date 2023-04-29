use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }

    let mut ans = 0;
    for (i, &c_i) in c.iter().enumerate() {
        if a + b == c_i {
            ans = i + 1;
        }
    }

    println!("{:?}", ans);
}
