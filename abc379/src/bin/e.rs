use itertools::Itertools as _;
use proconio::{input, marker::Chars};

/**
 * https://atcoder.jp/contests/abc379/tasks/abc379_e
 * https://atcoder.jp/contests/abc379/editorial/11311
 *
 * N が小さい値で実験してみると(N=3)、
 * f(1,1) + f(1,2) + f(1,3) + f(2,2) + f(2,3) + f(3,3)
 * = (S1) + (10S1 + S2) + (100S1 + 10S2 + S3) + (S2) + (10S2 + S3) + (S3)
 * = 10^2(S1) + 10^1(S1 + 2S2) + 10^0(S1 + 2S2 + 3S3)
 * となることがわかります。
 *
 */
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut sum = 0;
    let mut num_vec = vec![0; n];
    for i in 0..n {
        let num = (s[i] as u8 - b'0') as usize;
        sum += num * (i + 1);
        num_vec[i] = sum;
    }

    num_vec.reverse();

    let mut left = 0;
    let mut ans = vec![];
    for &cur_num in &num_vec {
        let num = cur_num + left;

        let digit_num = num % 10;
        left = num / 10;

        ans.push(std::char::from_digit(digit_num as u32, 10).unwrap());
    }

    while left > 0 {
        let digit_num = left % 10;
        left /= 10;
        ans.push(std::char::from_digit(digit_num as u32, 10).unwrap());
    }

    println!("{}", ans.iter().rev().join(""));
}
