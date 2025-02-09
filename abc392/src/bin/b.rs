use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut c = vec![];
    for i in 1..=n {
        if !a.contains(&i) {
            c.push(i);
        }
    }

    println!("{}", c.len());
    println!("{}", c.iter().join(" "));
}
