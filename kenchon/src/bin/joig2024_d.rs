use proconio::input;
use superslice::*;

/// D - たくさんの数字 (Many Digits)
/// https://atcoder.jp/contests/joig2024-open/tasks/joig2024_d
/// https://www2.ioi-jp.org/joig/2023/2024-ho/2024-joig-ho-t4-review.pdf
///
/// 解けたけど実装がめちゃくちゃハマった
/// => 桁を固定して、桁ごとに足してその桁になる数字を抽出していく
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort();
    b.sort();

    let get_digit = |num: usize| -> usize {
        let mut digit = 0;
        let mut num = num;
        while num > 0 {
            digit += 1;
            num /= 10;
        }
        digit
    };

    let mut ans: usize = 0;
    for &a_i in &a {
        let digit = get_digit(a_i);
        let target = 10_i32.pow(digit as u32) as usize;
        let digit_up_num = target - a_i;

        let digit_up_num_i = b.lower_bound(&digit_up_num);
        let base_num_i = b.upper_bound(&a_i);

        if base_num_i > digit_up_num_i {
            let digit_up_cnt = base_num_i - digit_up_num_i;
            ans += (digit + 1) * digit_up_cnt;
            ans += digit * digit_up_num_i;
        } else {
            ans += digit * base_num_i;
        }
    }

    for &b_i in &b {
        let digit = get_digit(b_i);
        let target = 10_i32.pow(digit as u32) as usize;
        let digit_up_num = target - b_i;

        let digit_up_num_i = a.lower_bound(&digit_up_num);
        let base_num_i = a.lower_bound(&b_i); // 5, 5 ← 片方しか含まないようにする

        if base_num_i > digit_up_num_i {
            let digit_up_cnt = base_num_i - digit_up_num_i;
            ans += (digit + 1) * digit_up_cnt;
            ans += digit * digit_up_num_i;
        } else {
            ans += digit * base_num_i;
        }
    }

    println!("{}", ans);
}
