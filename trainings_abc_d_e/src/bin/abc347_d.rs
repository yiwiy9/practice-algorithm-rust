use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut c_chars = decimal_to_chars(c, 2);
    while c_chars.len() < 60 {
        c_chars.insert(0, '0');
    }
    let mut same_count = 0;
    let mut diff_count = 0;
    for &c_char in &c_chars {
        if c_char == '0' {
            same_count += 1;
        } else {
            diff_count += 1;
        }
    }

    let mut a_on_from_diff_count = -1;
    let mut ab_on_from_same_count = -1;
    for i in 0..=same_count {
        for j in 0..=diff_count {
            if i + j == a && i + diff_count - j == b {
                a_on_from_diff_count = j as i32;
                ab_on_from_same_count = i as i32;
            }
        }
    }

    if a_on_from_diff_count == -1 || ab_on_from_same_count == -1 {
        println!("-1");
        return;
    }

    let mut a_chars = vec!['0'; 60];
    let mut b_chars = vec!['0'; 60];
    for (i, &c_char) in c_chars.iter().enumerate() {
        if c_char == '0' {
            if ab_on_from_same_count > 0 {
                a_chars[i] = '1';
                b_chars[i] = '1';
                ab_on_from_same_count -= 1;
            }
        } else if a_on_from_diff_count > 0 {
            a_chars[i] = '1';
            a_on_from_diff_count -= 1;
        } else {
            b_chars[i] = '1';
        }
    }

    println!(
        "{} {}",
        chars_to_decimal(a_chars, 2),
        chars_to_decimal(b_chars, 2)
    );
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
