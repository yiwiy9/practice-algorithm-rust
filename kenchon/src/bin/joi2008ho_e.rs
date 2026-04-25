use std::collections::VecDeque;

use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   ベニヤ板全体ではなく、テープの辺で分割された小長方形セルごとの話にして、各セルがテープで覆われているかだけ持てばよい。
/// - それについて、何が分かれば答えになる？
///   覆われていないセル同士の4近傍連結成分数が分かれば、必要なペンキの色数になる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   テープの端の座標をまたがない範囲では覆われ方は変化しないので、元の巨大な座標平面やテープを貼る途中経過は不要。
///   圧縮後のセル `(i, j)` は `[xs[i], xs[i+1]) x [ys[j], ys[j+1])` を表すので、面積ではなく隣接関係だけ保てばよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   `0, w, x1, x2` と `0, h, y1, y2` を座標圧縮し、各テープを圧縮 index の長方形として2次元 imosに加算し、累積和で覆われたセルを復元してから未被覆セルを BFS/DFS で数える。
///
/// 解説AC
/// こういう色塗りの問題はグラフ探索したいな→h,wの制約的にダメだな→座標圧縮したら解決しそう
/// までは一瞬考えたが、座標圧縮した後のセルの取り扱い方が思いつかなかった
/// 今回のポイントは、座標圧縮後に「点」を扱うのではなく、 `隣り合う圧縮座標の間の区間を1セルとして扱う` こと
fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        xy12: [(usize,usize,usize,usize); n],
    }

    // 座標圧縮
    let mut x = vec![0, w];
    let mut y = vec![0, h];
    for &(x1, y1, x2, y2) in &xy12 {
        x.push(x1);
        x.push(x2);
        y.push(y1);
        y.push(y2);
    }
    x.sort();
    x.dedup();
    y.sort();
    y.dedup();

    let w_len = x.len();
    let h_len = y.len();
    let mut s = vec![vec![0; w_len]; h_len];

    // 二次元いもす法
    for &(x1, y1, x2, y2) in &xy12 {
        let lx = x.lower_bound(&x1);
        let ly = y.lower_bound(&y1);
        let rx = x.lower_bound(&x2);
        let ry = y.lower_bound(&y2);
        s[ly][lx] += 1;
        s[ly][rx] -= 1;
        s[ry][lx] -= 1;
        s[ry][rx] += 1;
    }
    for row in 0..h_len {
        for col in 1..w_len {
            s[row][col] += s[row][col - 1];
        }
    }
    for row in 1..h_len {
        for col in 0..w_len {
            s[row][col] += s[row - 1][col];
        }
    }

    // 連結成分の個数
    let mut ans = 0;
    let cell_h = h_len - 1;
    let cell_w = w_len - 1;
    for row in 0..cell_h {
        for col in 0..cell_w {
            if s[row][col] != 0 {
                continue;
            }
            grid_bfs(&mut s, cell_h, cell_w, row, col);
            ans += 1;
        }
    }

    println!("{}", ans);
}

const DX: [i16; 4] = [1, 0, -1, 0];
const DY: [i16; 4] = [0, 1, 0, -1];

// MLE対策で seen を持たず、field 自体に訪問済みを入れる
pub fn grid_bfs(field: &mut [Vec<i16>], h: usize, w: usize, row: usize, col: usize) {
    let mut que = VecDeque::new();
    field[row][col] = -1;
    que.push_back((row as u16, col as u16));

    while let Some((row, col)) = que.pop_front() {
        let row = row as usize;
        let col = col as usize;

        for dir in 0..4 {
            let nrow = row as i16 + DX[dir];
            let ncol = col as i16 + DY[dir];
            if nrow < 0 || h as i16 <= nrow || ncol < 0 || w as i16 <= ncol {
                continue;
            }
            let nrow = nrow as usize;
            let ncol = ncol as usize;
            if field[nrow][ncol] != 0 {
                continue;
            }
            field[nrow][ncol] = -1;
            que.push_back((nrow as u16, ncol as u16));
        }
    }
}
