use proconio::input;

/**
 * https://atcoder.jp/contests/abc363/tasks/abc363_d
 * https://atcoder.jp/contests/abc363/editorial/10464
 *
 * 丁寧に実験して、考察を積み上げるしかない
 */
fn main() {
    input! {
        mut n: usize,
    }

    if n == 1 {
        println!("0");
        return;
    }

    n -= 1;

    for d in 1.. {
        let x = (d + 1) / 2;

        if n > 9 * 10usize.pow(x - 1) {
            n -= 9 * 10usize.pow(x - 1);
            continue;
        }

        let front_chars = decimal_to_chars(10usize.pow(x - 1) + n - 1, 10);
        let back_chars = front_chars.iter().rev().copied().collect::<Vec<char>>();

        println!(
            "{}",
            front_chars
                .iter()
                .chain(back_chars.iter().skip((d % 2) as usize))
                .collect::<String>()
        );
        return;
    }
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
