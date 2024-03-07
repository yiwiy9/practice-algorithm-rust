use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let n_chars = decimal_to_chars(n, 2);

    let mut ans = 0;
    for &c in n_chars.iter().rev() {
        if c == '0' {
            ans += 1;
        } else {
            break;
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
