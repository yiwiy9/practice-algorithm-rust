use itertools::Itertools;
use proconio::input;

/// https://atcoder.jp/contests/abc445/tasks/abc445_f
/// https://atcoder.jp/contests/abc445/editorial/15907
///
/// ============================================
/// 解法メモ：min-plus 行列累乗による
/// 「ちょうどK歩の最短経路」
/// ============================================
///
/// ■ 問題の本質
///
/// dp[k][i][j] を
/// 「頂点 i から j にちょうど k 歩で行く最小コスト」
/// と定義する。
///
/// ■ 通常のDP遷移
///
/// k+1 歩で i → j に行くには、
/// 最後の1歩の直前に必ずどこかの頂点 t を通る。
///
/// よって:
///
/// dp[k+1][i][j]
///   = min_t ( dp[k][i][t] + C[t][j] )
///
/// これは完全に自然なDPである。
///
/// ■ 行列として見る
///
/// 上の式は次の形になっている：
///
///   min_t ( A[i][t] + B[t][j] )
///
/// これは通常の行列積:
///
///   Σ_t ( A[i][t] * B[t][j] )
///
/// と形が同じで、
///
///   Σ → min
///   * → +
///
/// に置き換わったものと見なせる。
///
/// この演算を「min-plus 行列積」と呼ぶ。
///
/// 定義:
///
/// (A ⊗ B)[i][j]
///   = min_t ( A[i][t] + B[t][j] )
///
/// よって:
///
/// dp[k+1] = dp[k] ⊗ C
///
/// となる。
///
/// ■ 何を高速化しているのか？
///
/// 通常は:
///
/// dp[1]
/// dp[2]
/// ...
/// dp[K]
///
/// と K 回DPを回す必要があり、
/// 計算量は O(K * N^3) となる。
///
/// しかし:
///
/// dp[K] = C ⊗ C ⊗ ... ⊗ C  (K回)
///
/// と書けるため、
/// これは「行列のK乗」と同じ形になる。
///
/// ここで繰り返し二乗法を使えば:
///
///   O(N^3 log K)
///
/// で計算可能になる。
///
/// ■ 本質
///
/// これは特別な数学ではなく、
/// 「同じDP遷移をK回繰り返す問題」を
/// 累乗で高速化しているだけである。
///
/// 三重ループの min 更新を
/// “行列の掛け算” と呼んでいるだけ。
///
/// ■ このテクニックが解決する問題
///
/// 1. 同じ遷移をK回繰り返す問題
///
///   - ちょうどK回移動
///   - 長さKの経路
///   - K回操作後の状態
///   - Kステップ後のDP
///
/// 2. K が非常に大きい問題
///
///   - K ≤ 10^12
///   - K ≤ 10^18
///
///   通常のDPでは不可能だが、
///   累乗により log K に圧縮できる。
///
/// 3. 状態数が小さい問題
///
///   行列積は O(N^3) なので
///   N ≤ 50〜60 程度が現実的。
///
/// ============================================
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [[usize; n]; n],
    }

    let res = tropical_min_plus_pow(c.clone(), k);

    println!(
        "{}",
        res.iter().enumerate().map(|(i, res_i)| res_i[i]).join("\n")
    );
}

/// min-plus（tropical）積（一般形）:
/// C = A ⊗ B
/// C[i][j] = min_t (A[i][t] + B[t][j])
/// 次元:
/// A: n×m, B: m×p, C: n×p
pub fn tropical_min_plus_mul(a: &[Vec<usize>], b: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = a.len();
    debug_assert!(n > 0);
    let m = a[0].len();
    debug_assert!(a.iter().all(|row| row.len() == m));
    debug_assert!(b.len() == m);
    let p = b[0].len();
    debug_assert!(b.iter().all(|row| row.len() == p));
    let mut ret = vec![vec![TROPICAL_MINPLUS_INF_USIZE; p]; n];
    for i in 0..n {
        for t in 0..m {
            let x = a[i][t];
            if x >= TROPICAL_MINPLUS_INF_USIZE {
                continue;
            }
            for j in 0..p {
                let y = b[t][j];
                if y >= TROPICAL_MINPLUS_INF_USIZE {
                    continue;
                }
                let v = x.saturating_add(y).min(TROPICAL_MINPLUS_INF_USIZE);
                if v < ret[i][j] {
                    ret[i][j] = v;
                }
            }
        }
    }
    ret
}
/// min-plus（tropical）用の「無限大」
/// 衝突しにくいように prefix を付ける。
/// (1<<60) を使い、saturating_add + min(INF) で overflow を避ける。
const TROPICAL_MINPLUS_INF_USIZE: usize = 1usize << 60;
/// min-plus（tropical）の単位行列（N×N）:
/// I[i][i] = 0, I[i][j] = INF (i != j)
/// ※ 累乗で必要になるのはこの「正方」単位行列。
fn tropical_min_plus_identity(n: usize) -> Vec<Vec<usize>> {
    let mut id = vec![vec![TROPICAL_MINPLUS_INF_USIZE; n]; n];
    for i in 0..n {
        id[i][i] = 0;
    }
    id
}
/// min-plus（tropical）行列累乗（繰り返し二乗法）:
/// A^(exp)（⊗ による累乗）を返す。
/// 前提: A は N×N（正方行列）
/// exp=0 のときは単位行列を返す。
pub fn tropical_min_plus_pow(mut a: Vec<Vec<usize>>, mut exp: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    debug_assert!(a.iter().all(|row| row.len() == n));
    let mut res = tropical_min_plus_identity(n);
    while exp > 0 {
        if (exp & 1) == 1 {
            res = tropical_min_plus_mul(&res, &a);
        }
        a = tropical_min_plus_mul(&a, &a);
        exp >>= 1;
    }
    res
}
