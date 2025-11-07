use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut c_chars = decimal_to_chars(c, 2);
    if c_chars.len() < 60 {
        let mut pad = vec!['0'; 60 - c_chars.len()];
        pad.extend(c_chars);
        c_chars = pad;
    }
    let c_on_cnt = c_chars.iter().filter(|&&c| c == '1').count();
    let c_off_cnt = c_chars.len() - c_on_cnt;

    for same_one_cnt in 0..=c_off_cnt {
        if same_one_cnt > a || same_one_cnt > b {
            break;
        }

        let a_diff_one_cnt = a - same_one_cnt;
        let b_diff_one_cnt = b - same_one_cnt;
        if a_diff_one_cnt + b_diff_one_cnt != c_on_cnt {
            continue;
        }

        let mut used_same_one_cnt = 0;
        let mut used_a_diff_one_cnt = 0;
        let mut a_chars = vec!['0'; 60];
        let mut b_chars = vec!['0'; 60];
        for (i, &c) in c_chars.iter().enumerate() {
            if c == '0' {
                if used_same_one_cnt < same_one_cnt {
                    a_chars[i] = '1';
                    b_chars[i] = '1';
                    used_same_one_cnt += 1;
                }
            } else if used_a_diff_one_cnt < a_diff_one_cnt {
                a_chars[i] = '1';
                used_a_diff_one_cnt += 1;
            } else {
                b_chars[i] = '1';
            }
        }

        println!(
            "{} {}",
            chars_to_decimal(a_chars, 2),
            chars_to_decimal(b_chars, 2)
        );
        return;
    }

    println!("-1");
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
