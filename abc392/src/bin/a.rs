use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }

    println!(
        "{}",
        if a1 * a2 == a3 || a1 * a3 == a2 || a2 * a3 == a1 {
            "Yes"
        } else {
            "No"
        }
    );
}
