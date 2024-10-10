use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, usize); n-1],
    }

    uvw.sort_by(|a, b| a.2.cmp(&b.2));

    let mut ans = 0;
    let mut dsu = Dsu::new(n);
    for &(u, v, w) in &uvw {
        ans += w * dsu.size(u) * dsu.size(v);
        dsu.merge(u, v);
    }

    println!("{}", ans);
}
