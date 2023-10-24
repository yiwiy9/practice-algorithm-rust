use proconio::input;
const INF: i32 = 1 << 30;

/**
 * https://atcoder.jp/contests/abc178/tasks/abc178_e
 * https://img.atcoder.jp/abc178/editorial-E-phcdiydzyqa.pdf
 *
 * 【45度回転】(マンハッタン距離を扱う問題でよく使われる変形)
 * |xi − xj| + |yi − yj| = max(|xi + yi − (xj + yj)|, |xi − yi − (xj − yj)|)
 * zi = xi + yi, wi = xi − yi とすると、
 * |xi − xj| + |yi − yj| = max(|zi − zj|, |wi − wj|)
 *
 * よって、
 * max(|xi − xj| + |yi − yj|) = max(max(|zi − zj|, |wi − wj|))
 *                            = max(max(zi) - min(zj), max(wi) - min(wi))
 */
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut z_max = -INF;
    let mut z_min = INF;
    let mut w_max = -INF;
    let mut w_min = INF;

    for (x, y) in xy {
        z_max = z_max.max(x + y);
        z_min = z_min.min(x + y);
        w_max = w_max.max(x - y);
        w_min = w_min.min(x - y);
    }

    println!("{}", (z_max - z_min).max(w_max - w_min));
}
