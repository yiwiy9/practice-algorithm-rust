use proconio::input;

/// https://atcoder.jp/contests/joisc2013-day1/tasks/joisc2013_joi_poster
/// https://imoz.jp/data/joi/2013-sp-d1-joi_poster.pdf
/// https://drken1215.hatenablog.com/entry/2020/12/03/195200
/// 実数型の数値を比較するときは誤差に気を付けるため、EPS を用いるのが定石
fn main() {
    input! {
        n: usize,
        w: f64,
        h: f64,
        xy: [(f64, f64); n],
    }

    // pairwise distance（sqrt）を前計算すると楽＆速い
    let mut dist = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            let dx = xy[i].0 - xy[j].0;
            let dy = xy[i].1 - xy[j].1;
            dist[i][j] = (dx * dx + dy * dy).sqrt();
        }
    }

    let eps = 1e-8;

    let inside_rect = |cx: f64, cy: f64, r: f64| -> bool {
        let d = cx.min(w - cx).min(cy).min(h - cy);
        r <= d + eps // 辺に接するのはOK
    };

    let mut ans: i64 = 0;

    for a in 0..n {
        for b in 0..n {
            if b == a {
                continue;
            }
            let r1 = dist[a][b];
            if !inside_rect(xy[a].0, xy[a].1, r1) {
                continue;
            }

            for c in 0..n {
                if c == a || c == b {
                    continue;
                }
                let d_ac = dist[a][c];

                for d in 0..n {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    let r2 = dist[c][d];
                    if !inside_rect(xy[c].0, xy[c].1, r2) {
                        continue;
                    }

                    // 内部に含む（内接NG）
                    if r1 - r2 > d_ac + eps {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{ans}");
}
