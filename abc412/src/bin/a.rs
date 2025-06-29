use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    println!("{}", (0..n).filter(|&i| ab[i].0 < ab[i].1).count());
}
