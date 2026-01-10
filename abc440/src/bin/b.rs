use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let mut t_i_vec = t
        .iter()
        .enumerate()
        .map(|(i, t_i)| (t_i, i + 1))
        .collect_vec();

    t_i_vec.sort();

    println!("{} {} {}", t_i_vec[0].1, t_i_vec[1].1, t_i_vec[2].1);
}
