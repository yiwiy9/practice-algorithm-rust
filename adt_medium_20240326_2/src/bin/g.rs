use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240326_2/tasks/abc227_d
 * https://atcoder.jp/contests/adt_medium_20240326_2/editorial/2911
 * どっひゃーなんと二分探索です
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = (1_usize << 60) / k;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let sum = a.iter().map(|&x| x.min(mid)).sum::<usize>();
        if sum >= mid * k {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
