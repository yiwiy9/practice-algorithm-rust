use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.iter()
            .map(|&a_i| {
                if a_i <= l {
                    return l;
                } else if r <= a_i {
                    return r;
                } else {
                    return a_i;
                }
            })
            .join(" ")
    );
}
