use itertools::Itertools;
use proconio::{input, marker::Chars};
const MAX: usize = 1_000_000_000_000_000_000;

/// https://atcoder.jp/contests/abc450/tasks/abc450_e
/// https://atcoder.jp/contests/abc450/editorial/17731
/// S3 = S1 + S2 と誤読、本来は S3 = S2 + S1 で先頭は固定
/// => なんか複雑すぎると思ったら誤読を疑え
///
/// フィボナッチ数と項数の関係は、ほぼ O(logN) （2^Nより変化量が小さいが動きは似てる）
/// 今回は高々 88 項考えれば良い
///
/// L 文字目から R 文字目までの個数は「先頭から R 文字目までの個数」から「先頭から L−1 文字目までの個数」を引いたもの
/// 「先頭から N 文字目までの個数は？」という形の問題を 2 回解けばもとの問題に答えることができる
///
/// ↑ 本番もここまでは誤読した中で考察は進んだ
///   では、これをどのように再帰に落とし込む？ここの考察が弱かった
///
/// f_c(k, n) = X の n 文字目までに含まれる c の個数      （k=1 のとき）
///             Y の n 文字目までに含まれる c の個数      （k=2 のとき）
///             f_c(k-1, n)                           （k>=3 かつ n<=|Sk-1| のとき）
///             |Sk-1| に含まれる c の個数 + f_c(k-1, n) （k>=3 かつ n>|Sk-1| のとき）
///
/// ↑ おそらく絵書くときに誤読してなかったら視覚的に気づけたはず
fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        lrc: [(usize,usize,char); q],
    }

    let mut x_c_cnt = vec![vec![0; 26]; x.len() + 1];
    for i in 0..x.len() {
        for j in 0..26 {
            x_c_cnt[i + 1][j] = x_c_cnt[i][j]
                + if (x[i] as usize - 'a' as usize) == j {
                    1
                } else {
                    0
                };
        }
    }
    let mut y_c_cnt = vec![vec![0; 26]; y.len() + 1];
    for i in 0..y.len() {
        for j in 0..26 {
            y_c_cnt[i + 1][j] = y_c_cnt[i][j]
                + if (y[i] as usize - 'a' as usize) == j {
                    1
                } else {
                    0
                };
        }
    }

    let total_x_c_cnt = x_c_cnt.last().unwrap().clone();
    let total_y_c_cnt = y_c_cnt.last().unwrap().clone();

    let mut s_c_cnt = vec![total_x_c_cnt.to_vec(), total_y_c_cnt.to_vec()];
    let mut char_cnt = total_y_c_cnt.iter().sum::<usize>();
    while char_cnt < MAX {
        let i_1 = s_c_cnt.len() - 1;
        let i_2 = s_c_cnt.len() - 2;
        let new = (0..26)
            .map(|j| s_c_cnt[i_1][j] + s_c_cnt[i_2][j])
            .collect_vec();
        char_cnt = new.iter().sum::<usize>();
        s_c_cnt.push(new);
    }

    let k = s_c_cnt.len();
    for &(l, r, c) in &lrc {
        let j = c as usize - 'a' as usize;
        let l_cnt = rec(&x_c_cnt, &y_c_cnt, &s_c_cnt, j, k, l - 1);
        let r_cnt = rec(&x_c_cnt, &y_c_cnt, &s_c_cnt, j, k, r);
        println!("{}", r_cnt - l_cnt);
    }
}

fn rec(
    x_c_cnt: &[Vec<usize>],
    y_c_cnt: &[Vec<usize>],
    s_c_cnt: &[Vec<usize>],
    c: usize,
    k: usize,
    n: usize,
) -> usize {
    if k == 0 {
        return x_c_cnt[n][c];
    }
    if k == 1 {
        return y_c_cnt[n][c];
    }

    let s_1_cnt = s_c_cnt[k - 1].iter().sum::<usize>();
    if n <= s_1_cnt {
        return rec(x_c_cnt, y_c_cnt, s_c_cnt, c, k - 1, n);
    }

    s_c_cnt[k - 1][c] + rec(x_c_cnt, y_c_cnt, s_c_cnt, c, k - 2, n - s_1_cnt)
}
