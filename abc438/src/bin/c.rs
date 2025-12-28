use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack = vec![];
    for &a_i in &a {
        if stack.is_empty() {
            stack.push((a_i, 1));
        } else if let Some((before_a, before_cnt)) = stack.pop() {
            if before_a == a_i {
                if before_cnt == 3 {
                    continue;
                } else {
                    stack.push((before_a, before_cnt + 1));
                }
            } else {
                stack.push((before_a, before_cnt));
                stack.push((a_i, 1));
            }
        }
    }

    println!("{}", stack.iter().map(|&(_, cnt)| cnt).sum::<usize>());
}
