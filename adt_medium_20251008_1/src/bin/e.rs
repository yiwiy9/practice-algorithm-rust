use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();
    b.dedup();

    println!(
        "{}",
        k * (k + 1) / 2 - b.iter().filter(|&&num| num <= k).sum::<usize>()
    );
}
