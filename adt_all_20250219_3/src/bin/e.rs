use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack = vec![];
    for &a_i in &a {
        stack.push(a_i);

        while stack.len() >= 2 {
            if stack[stack.len() - 2] == stack[stack.len() - 1] {
                stack.pop();
                let x = stack.pop().unwrap();
                stack.push(x + 1);
            } else {
                break;
            }
        }
    }

    println!("{}", stack.len());
}
