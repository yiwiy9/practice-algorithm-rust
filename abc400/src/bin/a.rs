use proconio::input;

fn main() {
    input! {
        a: i64,
    }

    println!("{}", if a > 400 || 400 % a != 0 { -1 } else { 400 / a });
}
