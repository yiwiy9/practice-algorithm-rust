use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }

    println!("{}", s.iter().filter(|&&s_i| s_i <= x).sum::<usize>());
}
