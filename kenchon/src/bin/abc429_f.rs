use ac_library::{Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

/// https://atcoder.jp/contests/abc429/tasks/abc429_f
/// https://atcoder.jp/contests/abc429/editorial/14274
///
/// ポイントは「最短経路では左に戻る必要がない」という観察
/// （3 行しかないので、左へ戻って右へ進む動きは最短になり得ない、という主張）
///
/// すると、左→右に列をなめるだけでよくなる
///
/// そこで「各列（3マスの ./#）」を、DP の遷移（写像）として表現する。
/// 各列 j の状態から、3×3 行列 M_j を作る：
///   M_j[to][from] = 列 j の中だけで from行 → to行 に移るための “縦移動回数(上下移動)” の最小値
///                  （移れなければ INF）
///   ※ 横移動（右へ進む）は含めない（後でまとめて足す）
///
/// 区間 [L..R] の行列 M_{L..R} は、列行列を左から順に合成したものになる：
///   (A と B をつないだ区間の合成)
///   (A は左区間、B は右区間)
///   C[to][from] = min_{mid∈{0,1,2}} ( A[mid][from] + B[to][mid] )
/// これは結合的なのでモノイドになり、セグメント木で扱える。
///
/// 実装：
/// - 葉：各列 j の M_j を計算して載せる（3行なので場合分けで作れる）
/// - クエリ：1マス反転 → その列の M_j を作り直して一点更新
/// - 根：全区間の行列 all を取り、縦移動最小は all[2][0]（(1,1)->(3,n)）
/// - 答え：到達不能なら -1、そうでなければ (n-1) + all[2][0]
///         （横移動は必ず n-1 回）
///
/// 計算量：各更新 O(log N)（マージは 3^3=27 定数）
fn main() {
    input! {
        n: usize,
        s: [Chars; 3],
        q: usize,
        rc: [(Usize1, Usize1); q],
    }

    // 各列の壁状態を 3bit の mask で持つ（bit r が 1 なら '#'）
    let mut mask = vec![0u8; n];
    for r in 0..3 {
        for c in 0..n {
            if s[r][c] == '#' {
                mask[c] |= 1 << r;
            }
        }
    }

    // セグ木に各列の行列を載せる
    let init: Vec<Mat> = (0..n).map(|c| col_matrix(mask[c])).collect();
    let mut seg: Segtree<VertMonoid> = Segtree::from(init);

    for (r, c) in rc {
        // 1マス反転 → その列の行列だけ作り直す
        mask[c] ^= 1 << r;
        seg.set(c, col_matrix(mask[c]));

        // 全区間の “縦移動回数” の最小値（from=row0 -> to=row2）
        let all = seg.all_prod();
        let vert = all[2][0];

        if vert == INF {
            println!("-1");
        } else {
            // 横移動は (1,1) から (3,n) まで必ず n-1 回
            let ans = (n as i64 - 1) + vert;
            println!("{}", ans);
        }
    }
}

const INF: i64 = 1_i64 << 60;
type Mat = [[i64; 3]; 3];

/// 各ノードが持つ 3×3 行列は、ある区間の「縦移動回数（上下移動）の最小値」を表す。
/// mat[to][from] = 区間の左端の列で row=from にいる状態から始めて、区間の右端の列で row=to にいる状態で終わるときの
///                “区間内で発生する縦移動回数”の最小値（不可能なら INF）
///
/// 横移動（右へ進む）は列数で固定回数なので、この行列には含めず最後にまとめて足す。
struct VertMonoid;

impl Monoid for VertMonoid {
    type S = Mat;

    /// 恒等要素：from==to なら縦移動0、from!=to は不可能（INF）
    fn identity() -> Self::S {
        let mut m = [[INF; 3]; 3];
        for i in 0..3 {
            m[i][i] = 0;
        }
        m
    }

    /// 区間 A の右に区間 B をつなげた区間の行列を作る。
    ///
    /// A: 左区間, B: 右区間
    /// 境界（Aの右端列 / Bの左端列）で row=k を共有してつなぐとき、
    ///   A: from=j -> k
    ///   B: k -> to=i
    /// の縦移動を足して最小を取る。
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut res = [[INF; 3]; 3];
        for to in 0..3 {
            for from in 0..3 {
                let mut best = INF;
                for mid in 0..3 {
                    let x = a[mid][from];
                    let y = b[to][mid];
                    if x == INF || y == INF {
                        continue; // どちらかが到達不能なら繋げない
                    }
                    best = best.min(x + y);
                }
                res[to][from] = best;
            }
        }
        res
    }
}

/// 1列ぶんの壁状態 mask（bit r が 1 ならその行は '#'）から、縦移動だけの行列を作る。
/// 列内は上下移動しかできないので、3行のパス長をそのまま入れればよい。
fn col_matrix(mask: u8) -> Mat {
    let open = [
        (mask & 1) == 0, // row0
        (mask & 2) == 0, // row1
        (mask & 4) == 0, // row2
    ];

    let mut m = [[INF; 3]; 3];

    // 同じ行に留まる（そのマスが通れる必要がある）
    for r in 0..3 {
        if open[r] {
            m[r][r] = 0;
        }
    }

    // 隣接行の上下移動（両方通れるときだけ可能）
    if open[0] && open[1] {
        m[0][1] = 1;
        m[1][0] = 1;
    }
    if open[1] && open[2] {
        m[1][2] = 1;
        m[2][1] = 1;
    }

    // 0 <-> 2 は 1 を通る必要がある
    if open[0] && open[1] && open[2] {
        m[0][2] = 2;
        m[2][0] = 2;
    }

    m
}
