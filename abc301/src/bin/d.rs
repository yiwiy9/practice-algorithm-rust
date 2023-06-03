use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s_tmp: Chars,
        n: usize,
    }

    let mut s = vec!['0'; 60 - s_tmp.len()];
    s.extend(s_tmp);

    let n_bit_tmp = decimal_to_chars(n, 2);
    let mut n_bit = vec!['0'; 60 - n_bit_tmp.len()];
    n_bit.extend(n_bit_tmp);

    let mut lb = 60;
    for i in (0..60).rev() {
        if s[i] != '?' && s[i] != n_bit[i] {
            lb = i;
        }
    }

    if lb == 60 {
        println!("{}", n);
        return;
    }

    for i in (0..=lb).rev() {
        if s[i] == '1' || n_bit[i] == '0' {
            // S > Nの場合
            continue;
        }

        s[i] = '0';
        for s_j in s.iter_mut().take(60).skip(i + 1) {
            if *s_j == '?' {
                *s_j = '1';
            }
        }
        for (j, s_j) in s.iter_mut().take(i).enumerate() {
            if *s_j == '?' {
                *s_j = n_bit[j];
            }
        }

        println!("{}", chars_to_decimal(s, 2));
        return;
    }
    println!("{}", -1);
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
