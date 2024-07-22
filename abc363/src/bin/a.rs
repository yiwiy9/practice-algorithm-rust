use proconio::input;

fn main() {
    input! {
        r: usize,
    }

    let a = if r < 100 {
        100 - r
    } else if r < 200 {
        200 - r
    } else {
        300 - r
    };

    println!("{}", a);
}
