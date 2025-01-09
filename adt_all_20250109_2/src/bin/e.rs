use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut ans = vec![];

    // Functional Graph
    let mut dsu = Dsu::new(n);
    for i in 0..n {
        if !dsu.same(i, a[i]) {
            dsu.merge(i, a[i]);
            continue;
        }

        ans.push(i);
        let mut v = a[i];
        while v != i {
            ans.push(v);
            v = a[v];
        }
        break;
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x + 1).join(" "));
}
