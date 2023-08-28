use itertools::Itertools;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc133/tasks/abc133_d
 * こういう問題は和を考える
 * A1+...+AN = X1+...+XN = S
 *
 * Ai = Xi/2 + Xi+1/2
 * => Xi + Xi+1 = 2*Ai
 *
 * X1 = S - (X2+...+XN)
 *    = S - 2*(A2+A4+...+AN-1) // 偶数項の足し算
 */
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let s = a.iter().sum::<i32>();
    let s_even = a
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, &a_i)| a_i)
        .sum::<i32>();
    let mut x = s - 2 * s_even;

    println!(
        "{}",
        a.iter()
            .map(|a_i| {
                let res = x;
                x = 2 * a_i - x;
                res
            })
            .join(" ")
    );
}
