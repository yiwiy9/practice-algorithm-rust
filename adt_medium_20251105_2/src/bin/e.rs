use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut ans = 0;
    let mut dsu = Dsu::new(n);
    for &(u, v) in &uv {
        if dsu.same(u, v) {
            ans += 1;
        } else {
            dsu.merge(u, v);
        }
    }

    println!("{}", ans);
}
