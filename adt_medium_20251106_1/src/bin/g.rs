use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![];
    for &s_i in &s {
        if s_i == 'C'
            && stack.len() >= 2
            && stack[stack.len() - 1] == 'B'
            && stack[stack.len() - 2] == 'A'
        {
            stack.pop();
            stack.pop();
        } else {
            stack.push(s_i);
        }
    }

    println!("{}", stack.iter().join(""));
}
