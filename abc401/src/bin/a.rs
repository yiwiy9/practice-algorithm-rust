use proconio::input;

fn main() {
    input! {
        s: usize,
    }

    println!(
        "{}",
        if 200 <= s && s < 300 {
            "Success"
        } else {
            "Failure"
        }
    );
}
