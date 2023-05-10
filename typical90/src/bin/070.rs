use proconio::input;

/**
 * https://twitter.com/e869120/status/1406020809492090882/photo/1
 * 絶対値最小は平均値ではなく中央値！！
 */
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut x_vec = vec![];
    let mut y_vec = vec![];
    for (x, y) in &xy {
        x_vec.push(x);
        y_vec.push(y);
    }

    x_vec.sort();
    y_vec.sort();

    let x_mid = x_vec[(n - 1) / 2];
    let y_mid = y_vec[(n - 1) / 2];

    let mut ans = 0;
    for (x, y) in &xy {
        ans += (x - x_mid).abs();
        ans += (y - y_mid).abs();
    }

    println!("{}", ans);
}
