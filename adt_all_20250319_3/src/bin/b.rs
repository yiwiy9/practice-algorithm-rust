use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }

    for i in 0..n {
        if c[i] == a + b {
            println!("{}", i + 1);
        }
    }
}
