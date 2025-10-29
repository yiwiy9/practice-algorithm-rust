use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut ans = 0;
    let mut dsu = Dsu::new(n);
    for &(a, b) in &ab {
        if dsu.same(a, b) {
            ans += 1;
            continue;
        }
        dsu.merge(a, b);
    }

    println!("{}", ans);
}
