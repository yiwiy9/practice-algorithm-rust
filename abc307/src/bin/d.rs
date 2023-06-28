use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut stack = vec![];
    let mut left_cnt = 0;

    for c in &s {
        match c {
            '(' => {
                left_cnt += 1;
                stack.push(c);
            }
            ')' => {
                if left_cnt > 0 {
                    while let Some(before_c) = stack.pop() {
                        if *before_c == '(' {
                            break;
                        }
                    }
                    left_cnt -= 1;
                } else {
                    stack.push(c);
                }
            }
            _ => stack.push(c),
        }
    }

    println!(
        "{}",
        stack
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
