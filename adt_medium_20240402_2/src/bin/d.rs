use proconio::{input, marker::Chars};

const DX: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const DY: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

fn main() {
    input! {
        n: i32,
        a: [Chars; n],
    }

    let mut ans = 0_usize;
    for (i, row) in a.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            for k in 0..8 {
                let mut num = vec![];
                num.push(c);

                let mut x = i as i32;
                let mut y = j as i32;
                for _ in 0..n - 1 {
                    x += DX[k];
                    y += DY[k];
                    num.push(a[((x + n) % n) as usize][((y + n) % n) as usize]);
                }

                ans = ans.max(chars_to_decimal(num, 10));
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
