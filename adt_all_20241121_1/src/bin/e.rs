use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut stack = vec![];
    for i in (1..=n).rev() {
        stack.push((i as i32, 0));
    }

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    c: char,
                }
                let (x, y) = stack[stack.len() - 1];
                match c {
                    'L' => stack.push((x - 1, y)),
                    'R' => stack.push((x + 1, y)),
                    'U' => stack.push((x, y + 1)),
                    'D' => stack.push((x, y - 1)),
                    _ => unreachable!(),
                }
            }
            2 => {
                input! {
                    p: usize,
                }
                let (x, y) = stack[stack.len() - p];
                println!("{} {}", x, y);
            }
            _ => unreachable!(),
        }
    }
}
