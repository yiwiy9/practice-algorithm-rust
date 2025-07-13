use proconio::input;

/// https://atcoder.jp/contests/abc414/tasks/abc414_d
/// https://atcoder.jp/contests/abc414/editorial/13451
///
/// 座標の左から見ていって、「幅X以内であれば一つの電波塔で賄う」を決めていって、電波塔の個数がM以上ならXを大きくする
/// という二分探索をやったがダメ！
/// => 幅のMAXを最小にすれば総和は最小になるとは限らない
///
/// 例：
/// 6 3
/// 1 4 5 8 11 12
///
/// 正しくは 5 になりますが、
/// 恐らく最小値の二分探索だと 7 になります
///
/// 正：{1},{4,5,8},{11,12}
/// 誤：{1,4},{5,8},{11,12}
///
/// 左から条件を満たした時に繋げるという決め打ちが誤り
/// => 4と5を引き離してしまう
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; n],
    }

    x.sort();

    let mut d = vec![];
    for i in 0..n - 1 {
        d.push(x[i + 1] - x[i]);
    }
    d.sort();

    let mut ans = 0;
    for i in 0..n - m {
        ans += d[i];
    }

    println!("{}", ans);
}
