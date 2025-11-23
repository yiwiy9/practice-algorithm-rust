use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }

    let a = x - z * y;

    println!(
        "{}",
        if a >= 0 && a % (z - 1) == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
