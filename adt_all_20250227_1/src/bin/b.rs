use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: Usize1,
        b: Usize1,
    }

    println!(
        "{}",
        if a + 1 == b || (b + 1) % 10 == a {
            "Yes"
        } else {
            "No"
        }
    );
}
