use proconio::input;

fn main() {
    input! {
        q: usize,
        y: i64,
        s: [String; q],
    }

    let mut left = -1;
    let mut right = y + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let fx = calc_reverse_polish_notation(&s, mid);

        if fx >= y {
            right = mid;
        } else {
            left = mid;
        }
    }

    let fx = calc_reverse_polish_notation(&s, right);

    println!("{}", if fx == y { right } else { -1 });
}

fn calc_reverse_polish_notation(s: &[String], x: i64) -> i64 {
    let mut stack: Vec<i64> = vec![];
    for s_i in s {
        match s_i.as_str() {
            "max" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a.max(b));
            }
            "min" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a.min(b));
            }
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            "X" => {
                stack.push(x);
            }
            _ => {
                // Vec<char>だとparseが使えないのでStringにしてparseする
                stack.push(s_i.parse().unwrap());
            }
        }
    }
    stack[0]
}
