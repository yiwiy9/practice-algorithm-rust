use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let max = a.iter().max().unwrap();

    println!("{}", a.iter().filter(|&&x| x != *max).max().unwrap());
}
