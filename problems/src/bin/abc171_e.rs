use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let all = a.iter().fold(0, |acc, &a_i| acc ^ a_i);
    let ans = a.iter().map(|&a_i| all ^ a_i).collect::<Vec<usize>>();

    println!("{}", ans.iter().join(" "));
}
