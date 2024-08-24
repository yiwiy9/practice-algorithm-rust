use proconio::input;
const INF: usize = 1 << 60;

/**
 * https://atcoder.jp/contests/abc246/tasks/abc246_d
 * https://atcoder.jp/contests/abc246/editorial/3701
 *
 * この問題での目標は、全ての 0≤i≤10^6 に対して f(i,j)≥N なる最小の非負整数 j を求め、
 * それら全てに対して minf(i,j) を求めることになります。
 * ここで、i1≤i2 であるとき、i=i1 である場合の j を j1、i=i2 である場合の j を j2 とすると、 j1≥j2 となります。
 * => i が増えると j は減るので、2重ループが不要で一つ前の j を使って探索を続けられるので、高速。
 */
fn main() {
    input! {
        n: usize,
    }

    let f = |a: usize, b: usize| -> usize { a * a * a + a * a * b + a * b * b + b * b * b };

    let mut ans = INF;
    let mut i = 0;
    let mut j = 1_000_000;
    // 両端から尺取り的な動き
    while i <= j {
        let x = f(i, j);
        if x >= n {
            ans = ans.min(x);
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }

    println!("{}", ans);
}
