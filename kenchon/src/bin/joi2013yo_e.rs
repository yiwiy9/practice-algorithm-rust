use proconio::input;
use superslice::Ext;

/// https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_e
/// - 固定するもの:
///   各魚の生息範囲の境界だけを見る。重なり数は x, y, d の各境界をまたぐときにしか変化しない。
/// - 欲しい量:
///   境界で分割された各小立方体ごとの「その領域を含む魚の数」。
///   その値が K 以上の小立方体の体積和が答え。
/// - 必要クエリ:
///   各直方体を圧縮座標上の半開区間に変換し、3 次元 imos で加算する。
///   その後 3 次元累積和で各小立方体の被覆数を復元する。
/// - 単調性 / 境界:
///   被覆数は境界の間では一定なので、元の座標空間を全部見る必要はない。
///   各軸の境界座標を集めて圧縮すれば十分。
fn main() {
    input! {
        n: usize,
        k: i64,
        xyd: [((i64,i64,i64),(i64,i64,i64)); n],
    }

    let mut x = vec![];
    let mut y = vec![];
    let mut d = vec![];
    for &((x1, y1, d1), (x2, y2, d2)) in &xyd {
        x.push(x1);
        x.push(x2);
        y.push(y1);
        y.push(y2);
        d.push(d1);
        d.push(d2);
    }
    x.sort();
    x.dedup();
    y.sort();
    y.dedup();
    d.sort();
    d.dedup();

    let xx = x.len();
    let yy = y.len();
    let dd = d.len();

    let mut imos = vec![vec![vec![0_i64; dd + 1]; yy + 1]; xx + 1];
    for &((x1, y1, d1), (x2, y2, d2)) in &xyd {
        let x1_compressed = x.lower_bound(&x1);
        let y1_compressed = y.lower_bound(&y1);
        let d1_compressed = d.lower_bound(&d1);
        let x2_compressed = x.lower_bound(&x2);
        let y2_compressed = y.lower_bound(&y2);
        let d2_compressed = d.lower_bound(&d2);

        // 3次元いもす法
        // 加算記録 (8箇所)
        imos[x1_compressed][y1_compressed][d1_compressed] += 1;
        imos[x1_compressed][y1_compressed][d2_compressed] -= 1;
        imos[x1_compressed][y2_compressed][d1_compressed] -= 1;
        imos[x1_compressed][y2_compressed][d2_compressed] += 1;
        imos[x2_compressed][y1_compressed][d1_compressed] -= 1;
        imos[x2_compressed][y1_compressed][d2_compressed] += 1;
        imos[x2_compressed][y2_compressed][d1_compressed] += 1;
        imos[x2_compressed][y2_compressed][d2_compressed] -= 1;
    }

    for i in 0..xx {
        for j in 0..=yy {
            for l in 0..=dd {
                imos[i + 1][j][l] += imos[i][j][l];
            }
        }
    }
    for i in 0..=xx {
        for j in 0..yy {
            for l in 0..=dd {
                imos[i][j + 1][l] += imos[i][j][l];
            }
        }
    }
    for i in 0..=xx {
        for j in 0..=yy {
            for l in 0..dd {
                imos[i][j][l + 1] += imos[i][j][l];
            }
        }
    }

    let mut ans = 0;
    for i in 0..xx - 1 {
        for j in 0..yy - 1 {
            for l in 0..dd - 1 {
                if imos[i][j][l] < k {
                    continue;
                }
                ans += (x[i + 1] - x[i]) * (y[j + 1] - y[j]) * (d[l + 1] - d[l]);
            }
        }
    }

    println!("{}", ans);
}
