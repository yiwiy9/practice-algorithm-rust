use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b: Vec<&usize> = a.iter().unique().collect::<Vec<_>>();
    b.sort();

    let mut ans = 0;
    for i in 0..k {
        if i >= b.len() || *b[i] != i {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
