use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if x < y {
        println!("{}", if y - x <= 2 { "Yes" } else { "No" });
    } else {
        println!("{}", if x - y <= 3 { "Yes" } else { "No" });
    }
}
