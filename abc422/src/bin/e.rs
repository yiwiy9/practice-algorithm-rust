use proconio::input;

/// https://atcoder.jp/contests/abc422/tasks/abc422_e
/// https://atcoder.jp/contests/abc422/editorial/13820
/// この問題は 乱択アルゴリズム を使用することで高速かつ十分高確率で問題を解くことが出来ます。
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    for _ in 0..100 {
        let i = rand::random::<usize>() % n;
        let ii = rand::random::<usize>() % n;
        let a = xy[i].1 - xy[ii].1;
        let b = xy[ii].0 - xy[i].0;
        let c = xy[i].0 * xy[ii].1 - xy[i].1 * xy[ii].0;
        if a == 0 && b == 0 && c == 0 {
            continue;
        }

        let mut cnt = 0;
        for &(x, y) in &xy {
            if a * x + b * y + c == 0 {
                cnt += 1;
            }
        }

        if cnt * 2 > n {
            println!("Yes");
            println!("{} {} {}", a, b, c);
            return;
        }
    }

    println!("No");
}
