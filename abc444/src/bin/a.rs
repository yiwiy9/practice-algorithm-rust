use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        if n % 10 == (n / 10) % 10 && n % 10 == (n / 100) % 10 {
            "Yes"
        } else {
            "No"
        }
    );
}
