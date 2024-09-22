use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        mut m: usize,
    }

    let mut a = vec![];
    for a_i in (0..=10).rev() {
        while 3_usize.pow(a_i) <= m {
            a.push(a_i);
            m -= 3_usize.pow(a_i);
        }
    }

    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}
