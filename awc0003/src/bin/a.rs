use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize,usize); n],
    }

    println!("{}", ab.iter().filter(|&&(a, b)| a * b >= k).count());
}
