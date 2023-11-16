use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().fold(0, |acc, a_i| acc + a_i - 1));
}
