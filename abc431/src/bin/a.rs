use proconio::input;

fn main() {
    input! {
        h: usize,
        b: usize,
    }

    println!("{}", h.saturating_sub(b));
}
