use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.iter().filter(|&&a_i| a_i >= l).collect::<Vec<_>>().len()
    );
}
