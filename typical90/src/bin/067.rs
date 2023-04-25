use proconio::{input, marker::Chars};

fn chars_to_decimal(n: Vec<char>, base: u64) -> u64 {
    let mut res = 0;
    let mut x = 1;
    for &c in n.iter().rev() {
        res += (c as u64 - '0' as u64) * x;
        x *= base;
    }
    res
}

fn decimal_to_chars(mut n: u64, base: u64) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut res = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        res.push(char);
        n /= base;
    }
    res.iter().rev().copied().collect()
}

fn main() {
    input! {
        mut n_chars: Chars,
        k: i32,
    }

    for _i in 0..k {
        let n_decimal = chars_to_decimal(n_chars, 8);
        n_chars = decimal_to_chars(n_decimal, 9)
            .iter()
            .map(|num| match num {
                '8' => '5',
                _ => *num,
            })
            .collect()
    }

    for (i, c) in n_chars.iter().enumerate() {
        if i == n_chars.len() - 1 {
            println!("{}", c);
        } else {
            print!("{}", c);
        }
    }
}
