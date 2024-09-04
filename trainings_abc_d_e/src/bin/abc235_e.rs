use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abc: [(Usize1,Usize1,usize); m],
        uvw: [(Usize1,Usize1,usize); q],
    }

    let mut edges = abc
        .iter()
        .enumerate()
        .map(|(i, &(a, b, c))| (c, false, i, a, b))
        .collect::<Vec<_>>();
    edges.extend(
        uvw.iter()
            .enumerate()
            .map(|(i, &(u, v, w))| (w, true, i, u, v)),
    );

    edges.sort();

    let mut dsu = Dsu::new(n);
    let mut ans = vec![false; q];
    for &(_, is_q, i, a, b) in &edges {
        // クラスカル法
        if dsu.same(a, b) {
            if is_q {
                ans[i] = false;
            }
            continue;
        }
        if is_q {
            ans[i] = true;
        } else {
            dsu.merge(a, b);
        }
    }

    for &yes in &ans {
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
