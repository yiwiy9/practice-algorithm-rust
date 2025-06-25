use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", if a + b == 0 { 1 } else { 0 });
}
