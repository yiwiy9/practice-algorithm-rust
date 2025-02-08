use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!(
        "{}",
        if l == 1 && r == 0 {
            "Yes"
        } else if l == 0 && r == 1 {
            "No"
        } else {
            "Invalid"
        }
    );
}
