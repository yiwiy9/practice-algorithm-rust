use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
         ac: [(usize,usize); n],
    }

    let mut iac = ac
        .iter()
        .enumerate()
        .map(|(i, &(a, c))| (i + 1, a, c))
        .collect::<Vec<_>>();

    iac.sort_by(|a, b| b.1.cmp(&a.1));

    let mut ans = vec![];
    let mut min_c = 1 << 30;
    for &(i, _, c) in &iac {
        if c <= min_c {
            ans.push(i);
            min_c = min_c.min(c);
        }
    }

    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
