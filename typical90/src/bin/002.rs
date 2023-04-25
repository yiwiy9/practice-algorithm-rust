use proconio::input;

const LEFT: char = '(';
const RIGHT: char = ')';

fn main() {
    input! {
        n: usize,
    }
    dfs(LEFT.to_string(), n);
}

fn dfs(s: String, n: usize) {
    if s.len() == n {
        let mut ok = true;
        let mut left_count = 0;
        let mut right_count = 0;
        for c in s.chars() {
            match c {
                LEFT => left_count += 1,
                RIGHT => right_count += 1,
                _ => unreachable!(),
            }
            ok &= left_count >= right_count;
        }
        ok &= left_count == right_count;

        if ok {
            for c in s.chars() {
                print!("{}", c);
            }
            println!();
        }

        return;
    }

    let mut left_s = s.clone();
    left_s.push(LEFT);
    dfs(left_s, n);

    let mut right_s = s;
    right_s.push(RIGHT);
    dfs(right_s, n);
}
