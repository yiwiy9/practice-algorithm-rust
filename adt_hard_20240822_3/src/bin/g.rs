use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans = vec![];
    let mut stack = vec![];
    for c in s {
        match c {
            '(' => {
                stack.push(c);
            }
            ')' => {
                if stack.is_empty() {
                    ans.push(')');
                } else {
                    let mut temp = vec![];
                    while let Some(top) = stack.pop() {
                        if top == '(' {
                            break;
                        }
                        temp.push(top);
                    }
                }
            }
            _ => {
                if stack.is_empty() {
                    ans.push(c);
                } else {
                    stack.push(c);
                }
            }
        }
    }

    if !stack.is_empty() {
        ans.extend(stack.iter());
    }

    println!("{}", ans.iter().collect::<String>());
}
