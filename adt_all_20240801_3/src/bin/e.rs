use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut dsu = Dsu::new(n);
    let mut ans = vec![];
    for i in 0..n {
        if !dsu.same(i, a[i]) {
            dsu.merge(i, a[i]);
            continue;
        }

        // サイクルができたら、サイクル内の頂点を集める
        ans.push(i + 1);
        let mut v = a[i];
        while v != i {
            ans.push(v + 1);
            v = a[v];
        }
        break;
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
