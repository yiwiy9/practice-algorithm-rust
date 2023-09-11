use ac_library::Dsu;
use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc218/tasks/abc218_e
 * 最小全域木, クラスカル法, UnionFind, Dsu
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(Usize1,Usize1,i64); m],
    }

    let compare = |&i: &(usize, usize, i64), &j: &(usize, usize, i64)| i.2.cmp(&j.2);
    abc.sort_by(compare);

    let mut ans = abc.iter().map(|(_, _, c)| *c).sum::<i64>();
    let mut dsu = Dsu::new(n);
    for &(a, b, c) in &abc {
        if dsu.same(a, b) {
            if c < 0 {
                // ここだけクラスカル法と違う
                ans -= c;
            }
            continue;
        }
        ans -= c;
        dsu.merge(a, b);
    }

    println!("{}", ans);
}
