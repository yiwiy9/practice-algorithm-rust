use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    println!(
        "{}",
        if n >= -(1 << 31) && n < (1 << 31) {
            "Yes"
        } else {
            "No"
        }
    );
}
