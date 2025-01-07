use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().filter(|&&a_i| a_i < p).count());
}
