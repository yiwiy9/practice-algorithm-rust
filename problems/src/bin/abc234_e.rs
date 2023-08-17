use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let digit_x = decimal_to_chars(x, 10).len();

    let mut ans = chars_to_decimal(vec!['1'; digit_x + 1], 10);
    for first in 1..10 {
        'outer: for diff in 0..10 {
            let mut positional = first;
            let mut cur = first;
            for _ in 0..digit_x - 1 {
                if positional + diff > 9 {
                    break 'outer;
                }
                positional += diff;
                cur *= 10;
                cur += positional;
            }
            if cur >= x {
                ans = ans.min(cur);
            }
        }

        'outer: for diff in 0..10 {
            let mut positional = first;
            let mut cur = first;
            for _ in 0..digit_x - 1 {
                if positional < diff {
                    break 'outer;
                }
                positional -= diff;
                cur *= 10;
                cur += positional;
            }
            if cur >= x {
                ans = ans.min(cur);
            }
        }
    }

    println!("{}", ans);
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
