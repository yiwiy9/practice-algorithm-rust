use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(Usize1,Usize1); n],
    }

    let compare = |&i: &(usize, usize), &j: &(usize, usize)| i.1.cmp(&j.1);
    lr.sort_by(compare);

    let mut ans = 1;
    let mut crush_front = lr[0].1;
    for &(l, r) in &lr {
        if l < crush_front + d {
            continue;
        }
        ans += 1;
        crush_front = r;
    }

    println!("{}", ans);
}
