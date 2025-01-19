use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut cur = 1;
    for i in 1..=1000 {
        cur *= i;
        if cur == x {
            println!("{}", i);
            return;
        }
    }
}
