use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if 1 <= n && n <= 125 {
        println!("4");
    } else if 126 <= n && n <= 211 {
        println!("6");
    } else {
        println!("8");
    }
}
