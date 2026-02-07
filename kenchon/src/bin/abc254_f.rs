use ac_library::{Monoid, Segtree};
use num::integer::gcd;
use proconio::{input, marker::Usize1};

struct Gcd;
impl Monoid for Gcd {
    type S = i64;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        gcd(*l, *r)
    }
}

/// https://atcoder.jp/contests/abc254/tasks/abc254_f
/// https://atcoder.jp/contests/abc254/editorial/4067
/// https://atcoder.jp/contests/abc254/editorial/4070
///
/// 数学（gcdの式変形）メモ
///
/// 目的：
///   たくさんある (A_i + B_j) の gcd を、そのまま全部見るのではなく、
///   「1個の代表 + Aの差分 + Bの差分」の gcd に落として高速化する。
///
/// 基本性質（ユークリッド互除法）：
///   gcd(x, y) = gcd(x, y - x)  （一般に gcd(x, y) = gcd(x, y - kx)）
///   → “片方からもう片方の倍を引いても gcd は変わらない”（余りとの最大公約数と等しい）
///
/// (1) Aを固定して B 側を差分にする：
///   gcd(Ax+B1, Ax+B2) = gcd(Ax+B1, (Ax+B2)-(Ax+B1)) = gcd(Ax+B1, B2-B1)
///   同様に3つ以上でも、順に引いていくと
///   gcd(Ax+B1, Ax+B2, ..., Ax+BN)
///     = gcd(Ax+B1, (B2-B1), (B3-B2), ..., (BN-B{N-1}))
///   ここで右側の差分は x に依存しないのが重要（Ax が消える）
///
/// (2) 今度は B を固定して A 側も差分にする（対称）：
///   gcd(A1+By, A2+By, ..., AN+By)
///     = gcd(A1+By, (A2-A1), (A3-A2), ..., (AN-A{N-1}))
///
/// (3) 結論（全部の和の gcd）：
///   {A_i + B_j の全て} の gcd は、次と等しい：
///   gcd( A1+B1,
///        (A2-A1), (A3-A2), ..., (AN-A{N-1}),
///        (B2-B1), (B3-B2), ..., (BN-B{N-1}) )
///
/// ABC254F では区間 [h1..h2], [w1..w2] のクエリが来るので、
///   答え = gcd( A[h1] + B[w1],
///              gcd( A差分配列の区間[h1..h2-1] ),
///              gcd( B差分配列の区間[w1..w2-1] ) )
/// に帰着できる。
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        b: [i64; n],
        queries: [(Usize1,Usize1,Usize1,Usize1); q],
    }

    let mut a_gcd_segtree = Segtree::<Gcd>::new(n);
    let mut b_gcd_segtree = Segtree::<Gcd>::new(n);
    for i in 1..n {
        a_gcd_segtree.set(i, a[i] - a[i - 1]);
        b_gcd_segtree.set(i, b[i] - b[i - 1]);
    }

    for &(h1, h2, w1, w2) in &queries {
        let mut ans = a[h1] + b[w1];
        ans = gcd(ans, a_gcd_segtree.prod(h1 + 1..=h2));
        ans = gcd(ans, b_gcd_segtree.prod(w1 + 1..=w2));
        println!("{}", ans);
    }
}
