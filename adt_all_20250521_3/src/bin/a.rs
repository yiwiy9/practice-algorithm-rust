use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7*n],
    }

    let mut ans = vec![];
    for i in 0..n {
        let mut sum = 0;
        for j in 0..7 {
            sum += a[i * 7 + j];
        }
        ans.push(sum);
    }

    println!("{}", ans.iter().join(" "));
}
