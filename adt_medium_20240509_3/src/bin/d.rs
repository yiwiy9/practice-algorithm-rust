use proconio::{input, marker::Chars};

const DX: [i64; 8] = [1, 1, 1, 0, -1, -1, -1, 0];
const DY: [i64; 8] = [1, 0, -1, -1, -1, 0, 1, 1];

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let mut number = vec![];
                number.push(a[i][j]);

                let mut x = i as i64;
                let mut y = j as i64;
                for _ in 0..(n - 1) {
                    x += DX[k] + n as i64;
                    x %= n as i64;
                    y += DY[k] + n as i64;
                    y %= n as i64;
                    number.push(a[x as usize][y as usize]);
                }

                ans = ans.max(chars_to_decimal(number, 10));
            }
        }
    }

    println!("{}", ans);
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
