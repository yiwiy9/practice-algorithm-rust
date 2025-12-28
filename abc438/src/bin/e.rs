use itertools::Itertools;
use proconio::{input, marker::Usize1};

const LOG_K: usize = 31;

/// https://atcoder.jp/contests/abc438/tasks/abc438_e
/// https://atcoder.jp/contests/abc438/editorial/14964
///
/// ポジション移動以外のダブリング（和のダブリング）ってどうやるんや？で止まって、Functional Graph に飛びついたが実装むずくて爆死
/// => ポジション移動のダブリングと追加する水の量のダブリングの２本を持つと解決する
///
/// P[d][i]: 人iがあるバケツを持っている状態から 2^d 回操作を行ったとき、そのバケツを持っている人の番号
/// Q[d][i]: 人iがあるバケツを持っている状態から 2^d 回操作を行ったとき、そのバケツに 2^d 回の操作で新たに入った水の量
///
/// ダブリングの基本は
/// 「2^d 回操作を行う」 <=> 「2^(d-1)回操作を行った後の状態に対して、2^(d-1)回操作を行う」
/// ということ
///
/// 上記の基本方針を押さえれば、
/// Q[d][i]: 2^(d-1)回操作を行った後の位置（P[d-1][i]）、水の量（Q[d-1][i]）であり、
///          2^(d-1)回操作を行った後の位置に対して、さらに2^(d-1)回操作を行う
///          つまり、Q[d-1][i] + Q[d-1][P[d-1][i]]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        tb: [(usize,Usize1); q],
    }

    // dp_p[d][i]: i番目の要素から2^d回遷移したときの到達地点
    let mut dp_p = vec![vec![0; n]; LOG_K];
    // dp_q[d][i]: i番目の要素から2^d回遷移したときに追加される水の量
    let mut dp_q = vec![vec![0; n]; LOG_K];

    // 初期条件
    dp_p[0][..n].copy_from_slice(&a[..n]);
    dp_q[0][..n].copy_from_slice(&(1..=n).collect_vec());

    // 遷移
    for d in 1..LOG_K {
        for i in 0..n {
            // 2^i = 2^(i-1) + 2^(i-1)
            dp_p[d][i] = dp_p[d - 1][dp_p[d - 1][i]];
            dp_q[d][i] = dp_q[d - 1][i] + dp_q[d - 1][dp_p[d - 1][i]];
        }
    }

    for &(t, b) in &tb {
        let mut ans: usize = 0;
        let mut i = b;
        for bit in 0..LOG_K {
            if t & (1 << bit) != 0 {
                ans += dp_q[bit][i];
                i = dp_p[bit][i];
            }
        }
        println!("{}", ans);
    }
}
