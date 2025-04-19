use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();

    println!("{}", a.iter().position(|&x| x == b[n - 2]).unwrap() + 1);
}
