use ac_library::Dsu;
use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc214/tasks/abc214_d
 * https://atcoder.jp/contests/abc214/editorial/2434
 * クラスカル法っぽく考える
 * 重みが小さい順に辺を追加していくと、新たに追加する辺は既存の辺よりも重みが大きいか等しいので、
 * 両端の連結成分の積に重みをかけたものの和をとっていく事で、答えが求まる
 *
 * 最短パスという言葉に惑わされたが、**木なので新たに追加する辺は必ず最短パスの一部となる**
 */
fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, usize); n-1],
    }

    uvw.sort_by_key(|x| x.2);

    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for (u, v, w) in uvw {
        let size_u = dsu.size(u);
        let size_v = dsu.size(v);
        ans += size_u * size_v * w;
        dsu.merge(u, v);
    }

    println!("{}", ans);
}
