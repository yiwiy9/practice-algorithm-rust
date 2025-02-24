use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![];
    for &c in &s {
        match c {
            '(' | '[' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    println!("No");
                    return;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    println!("No");
                    return;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    if !stack.is_empty() {
        println!("No");
        return;
    }
    println!("Yes");
}
