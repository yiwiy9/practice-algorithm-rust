use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = 0_usize;
    let mut stack: Vec<(usize, usize)> = vec![];
    for &a_i in &a {
        if stack.is_empty() || stack.last().unwrap().0 != a_i {
            stack.push((a_i, 1));
            cnt += 1;
            println!("{}", cnt);
        } else {
            let (num, c) = stack.pop().unwrap();
            if num == c + 1 {
                cnt -= c;
            } else {
                stack.push((num, c + 1));
                cnt += 1;
            }
            println!("{}", cnt);
        }
    }
}
