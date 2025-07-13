use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize,usize); n],
    }

    println!("{}", xy.iter().filter(|&(x, y)| x <= &l && &r <= y).count());
}
