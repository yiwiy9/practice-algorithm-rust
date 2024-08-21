use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = 0;
    let mut stack = vec![];
    for &a_i in &a {
        if stack.is_empty() {
            cnt += 1;
            stack.push((a_i, 1));
            println!("{}", cnt);
            continue;
        }

        if let Some((top, count)) = stack.last_mut() {
            if *top != a_i {
                cnt += 1;
                stack.push((a_i, 1));
            } else if *count + 1 == a_i {
                cnt -= *count;
                stack.pop();
            } else {
                cnt += 1;
                *count += 1;
            }
        }

        println!("{}", cnt);
    }
}
