use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut a_sq = 1;
    for _ in 0..b {
        a_sq *= a;
    }
    let mut b_sq = 1;
    for _ in 0..a {
        b_sq *= b;
    }

    println!("{}", a_sq + b_sq);
}
