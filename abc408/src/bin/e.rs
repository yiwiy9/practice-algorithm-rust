use ac_library::Dsu;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc408/tasks/abc408_e
/// https://atcoder.jp/contests/abc408/editorial/13159
///
/// 辺のラベル w が w OR x=x を満たす辺のみを通って頂点 1 から頂点 N まで到達できるか？という判定問題を考えます。
/// この判定問題が Yes になる x の集合を X とすると、 minX が問題の答えとなります。
fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut x = (1 << 30) - 1; // 30bitの全てのビットが立っている状態
    for k in (0..30).rev() {
        // 上のbitから使わないで済むなら使わない => minX

        x ^= 1 << k; // k番目のbitを0にする

        let mut dsu = Dsu::new(n);
        for &(u, v, w) in &uvw {
            if (w | x) == x {
                // w OR x = x ならば辺を追加
                dsu.merge(u, v);
            }
        }

        if !dsu.same(0, n - 1) {
            // 頂点1と頂点Nが同じ連結成分に属さないならば、k番目のbitを1に戻す
            x ^= 1 << k;
        }
    }

    println!("{}", x);
}
