use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!("{}", r - l + 1);
}
