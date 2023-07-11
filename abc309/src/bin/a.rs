use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!(
        "{}",
        if a + 1 == b && a % 3 != 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
