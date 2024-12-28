use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!(
        "{}",
        if x < 40 {
            (40 - x).to_string()
        } else if x < 70 {
            (70 - x).to_string()
        } else if x < 90 {
            (90 - x).to_string()
        } else {
            "expert".to_string()
        }
    );
}
