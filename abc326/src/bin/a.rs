use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if (x < y && y - x <= 2) || (x > y && x - y <= 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}
