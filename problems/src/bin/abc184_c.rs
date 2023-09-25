use proconio::input;

/**
 * https://atcoder.jp/contests/abc184/tasks/abc184_c
 * https://atcoder.jp/contests/abc184/editorial/339
 *
 * 1. a+b=c+d
 * 2. a−b=c−d
 * 3. ∣a−c∣+∣b−d∣≤3
 *
 * 1,2を使用するとパリティが同じ任意のマスに移動可能
 * => 3も合わせると、任意のマスに3手で進めることがわかる
 *
 * 0,1,2手で行けるか調べる
 * => 行かなかったら3手
 */
fn main() {
    input! {
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }

    let r = r2 - r1;
    let c = c2 - c1;

    if (r, c) == (0, 0) {
        println!("{}", 0);
    } else if r == c || r == -c || r.abs() + c.abs() <= 3 {
        println!("{}", 1);
    } else if (r + c) % 2 == 0 || (r + c).abs() <= 3 || (r - c).abs() <= 3 || r.abs() + c.abs() <= 6
    {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }
}
