use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let small = a / b;
    let large = a / b + 1;

    if (a - small * b).abs() < (a - large * b).abs() {
        println!("{}", small);
    } else {
        println!("{}", large);
    }
}
