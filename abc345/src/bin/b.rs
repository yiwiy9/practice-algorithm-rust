use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    println!(
        "{}",
        if x > 0 && x % 10 != 0 {
            x / 10 + 1
        } else {
            x / 10
        }
    );
}
