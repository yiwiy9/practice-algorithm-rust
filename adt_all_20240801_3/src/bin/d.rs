use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    for a_i in &a {
        let mut connected = vec![];
        for (j, &a_ij) in a_i.iter().enumerate() {
            if a_ij == 1 {
                connected.push(j + 1);
            }
        }
        println!("{}", connected.iter().join(" "));
    }
}
