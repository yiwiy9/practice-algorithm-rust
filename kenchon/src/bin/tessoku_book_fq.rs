use ac_library::{Min, Segtree};
use proconio::input;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fq
/// https://github.com/E869120/kyopro-tessoku/blob/main/editorial/final/cpp/answer_C19.cpp
///
/// - 追加で買う量は必ず (L - K) L（スタート時点で K L 満タンなので）
/// - 追加で必要な (L - K) 個の 1L を、後半の各 1km 区間に対応させて考える
/// - 1km 進むごとに 1L 必要なので、区間 (s-1 → s) に使う 1L は
///   必ず直前 K km 内（位置 [s-K, s-1]）のどこかで買われている必要がある（容量 K の制約）
/// - よって、その 1L の最小コストは「区間 [s-K, s-1] の最安単価」
/// - したがって答えは  sum_{s=K+1..L}  min_{x in [s-K, s-1]} price[x]
/// - もしある s で [s-K, s-1] にスタンドが1つも無ければ、その 1L を買えず到達不能 → -1
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        ac: [(usize, usize); n],
    }

    let mut min_value = vec![INF; l + 1];
    for &(a, c) in &ac {
        min_value[a] = min_value[a].min(c);
    }

    let min_segtree = Segtree::<Min<usize>>::from(min_value);

    let mut ans: usize = 0;
    for l_i in k + 1..=l {
        let min_v = min_segtree.prod(l_i - k..l_i);
        if min_v == INF {
            println!("-1");
            return;
        }
        ans += min_v;
    }

    println!("{}", ans);
}
