use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    print!("1");

    for _ in 0..n {
        print!("01");
    }

    println!();
}
