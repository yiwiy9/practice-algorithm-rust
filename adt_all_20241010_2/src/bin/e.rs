use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut min = 0;
    let mut cur = 0;
    for &a_i in &a {
        cur += a_i;
        min = min.min(cur);
    }

    println!("{}", cur + min.min(0).abs());
}
