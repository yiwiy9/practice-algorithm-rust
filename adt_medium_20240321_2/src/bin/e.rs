use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240321_2/tasks/abc227_c
 * https://atcoder.jp/contests/adt_medium_20240321_2/editorial/2906
 */
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for a in 1..=n {
        if a * a * a > n {
            break;
        }
        for b in a..=n {
            if a * b * b > n {
                break;
            }
            ans += n / (a * b) - b + 1;
        }
    }

    println!("{}", ans);
}
