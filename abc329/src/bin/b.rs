use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let &max_a = a.iter().max().unwrap();

    println!("{}", a.iter().filter(|&&a_i| a_i != max_a).max().unwrap());
}
