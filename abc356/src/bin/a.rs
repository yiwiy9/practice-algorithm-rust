use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let mut ans = vec![];
    for i in 1..=n {
        if i < l || i > r {
            ans.push(i);
        } else {
            ans.push(r - i + l);
        }
    }
    println!("{}", ans.iter().join(" "));
}
