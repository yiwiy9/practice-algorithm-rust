use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let max_h = h.iter().max().unwrap();

    println!("{}", h.iter().position(|&x| x == *max_h).unwrap() + 1);
}
