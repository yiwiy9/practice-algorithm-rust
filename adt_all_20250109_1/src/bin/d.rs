use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        a: Chars,
        b: Chars,
    }

    let a_k = chars_to_decimal(a, k);
    let b_k = chars_to_decimal(b, k);

    println!("{}", a_k * b_k);
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
