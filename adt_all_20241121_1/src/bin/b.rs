use proconio::input;

fn main() {
    input! {
        y: usize,
    }

    let a = if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else if y % 4 == 0 {
        true
    } else {
        false
    };

    println!("{}", if a { "366" } else { "365" });
}
