use proconio::input;

/**
 * https://atcoder.jp/contests/abc354/tasks/abc354_d
 * https://atcoder.jp/contests/abc354/editorial/10036
 * 二次元累積和
 * 求めたい答え f(A,B,C,D) = f(0,0,C,D) - f(0,0,C,B) - f(0,0,A,D) + f(0,0,A,B)
 * 図: Tile Pattern https://atcoder.jp/contests/abc331/editorial/7822
 *
 * あとは、負の数の割り算は避けたい（0方向に丸められるため）ので、0≤A, 0≤B とするために十分大きい4の倍数をはじめに足す
 */
fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        mut c: i64,
        mut d: i64,
    }

    let offset = 1_000_000_000;
    a += offset;
    b += offset;
    c += offset;
    d += offset;

    println!(
        "{}",
        calc_origin_base(c, d) - calc_origin_base(c, b) - calc_origin_base(a, d)
            + calc_origin_base(a, b)
    );
}

fn calc_origin_base(x: i64, y: i64) -> i64 {
    let down_row = (x / 4) * 4
        + if x % 4 == 0 {
            0
        } else if x % 4 == 1 {
            2
        } else {
            3
        };

    let up_row = (x / 4) * 4
        + if x % 4 == 0 {
            0
        } else if x % 4 == 1 {
            1
        } else if x % 4 == 2 {
            3
        } else {
            4
        };

    (y / 2) * (down_row + up_row) + if y % 2 == 1 { down_row } else { 0 }
}
