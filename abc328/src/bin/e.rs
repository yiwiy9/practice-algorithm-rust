use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1,Usize1,usize); m],
    }

    let mut ans = INF;

    // 組み合わせ全探索
    for using_edges in (0..m).combinations(n - 1) {
        let mut dsu = Dsu::new(n);
        let mut cur = 0;

        for &edge_j in &using_edges {
            let (u, v, w) = uvw[edge_j];

            if dsu.same(u, v) {
                cur = INF;
                break;
            }

            cur += w;
            cur %= k;
            dsu.merge(u, v);
        }

        ans = ans.min(cur);
    }

    println!("{:?}", ans);
}
