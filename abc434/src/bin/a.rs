use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let mut n = 1;
    loop {
        if 1000 * w < n * b {
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
