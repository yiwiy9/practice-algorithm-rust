use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = vec![];
    for &a_i in &a {
        if a_i % k == 0 {
            ans.push(a_i / k);
        }
    }

    println!("{}", ans.iter().join(" "));
}
