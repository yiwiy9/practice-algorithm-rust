use ac_library::Dsu;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc408/tasks/abc408_e
/// https://atcoder.jp/contests/abc408/editorial/13159
/// => 本問題における辺のコストは分割統治できない
/// => ダイクストラができないことに気づいたらすぐ別のアプローチを考える
/// => 例えば、答えを決め打ちするパターンで、それが条件を満たすか？という逆パターンを探る
///
/// 答えを決め打ちするパターンの一つの解法として、
/// まず全て満たす x (= 2^30 - 1)から始めて、各桁を0にできるか？というのを調べていく
/// というアプローチで最小値に辿り着ける
fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1,Usize1,usize); m],
    }

    let mut x = (1 << 30) - 1;

    // 大きい桁を0にできた方が嬉しい貪欲
    for k in (0..30).rev() {
        x ^= 1 << k;

        let mut dsu = Dsu::new(n);
        for &(u, v, w) in &uvw {
            if x | w == x {
                dsu.merge(u, v);
            }
        }

        if !dsu.same(0, n - 1) {
            x |= 1 << k;
        }
    }

    println!("{}", x);
}
