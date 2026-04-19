use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        }

        let a_solve = solve(&a);
        let b_solve = solve(&b);

        println!("{}", if a_solve == b_solve { "Yes" } else { "No" });
    }
}

fn solve(s: &[char]) -> Vec<char> {
    let mut stack = vec![];

    let mut x_cnt = 0;
    for &s_i in s {
        if s_i == 'x' && x_cnt < 2 {
            x_cnt += 1;
            continue;
        }
        if s_i == ')' && x_cnt == 2 {
            if let Some(&last_c) = stack.last() {
                if last_c == '(' {
                    stack.pop();
                    continue;
                }
            }
        }

        for _ in 0..x_cnt {
            stack.push('x');
        }
        stack.push(s_i);
        x_cnt = 0;
    }
    for _ in 0..x_cnt {
        stack.push('x');
    }

    stack
}
