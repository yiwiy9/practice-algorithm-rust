use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    }

    let mut ans = vec![];
    for &a_i in &a {
        if a_i < l {
            ans.push(l);
        } else if a_i > r {
            ans.push(r);
        } else {
            ans.push(a_i);
        }
    }

    println!("{}", ans.iter().join(" "));
}
