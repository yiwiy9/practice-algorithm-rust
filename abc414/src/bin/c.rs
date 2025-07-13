use proconio::input;

/// 12桁の数字の回文を作る方法
/// => 1~10^6までを全探索して、反転した文字列を結合する
fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let mut ans = 0;

    for num in 1..10 {
        if num > n {
            break;
        }
        let num_a = decimal_to_chars(num, a);
        if is_palindrome(&num_a) {
            ans += num;
        }
    }

    for half_num in 1_usize..1_000_000 {
        let half_num_chars = decimal_to_chars(half_num, 10);
        let reverse_half_num_chars = half_num_chars.iter().rev().copied().collect::<Vec<_>>();

        for c in 0..=10 {
            let mid_num = if c == 10 {
                vec![]
            } else {
                vec![std::char::from_digit(c as u32, 10).unwrap()]
            };

            let full_num_chars = half_num_chars
                .iter()
                .chain(mid_num.iter())
                .chain(reverse_half_num_chars.iter())
                .copied()
                .collect::<Vec<_>>();

            let full_num_10 = chars_to_decimal(full_num_chars, 10);
            if full_num_10 > n {
                continue;
            }

            let full_num_a = decimal_to_chars(full_num_10, a);
            if is_palindrome(&full_num_a) {
                ans += full_num_10;
            }
        }
    }

    println!("{}", ans);
}

fn is_palindrome(s: &Vec<char>) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}

pub fn chars_to_decimal(n: Vec<char>, base: usize) -> usize {
    let mut result = 0;
    let mut x = 1;
    for &c in n.iter().rev() {
        result += (c as usize - '0' as usize) * x;
        x *= base;
    }
    result
}
