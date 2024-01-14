use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut history = vec![(1, 0)];
    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    c: char,
                }
                match c {
                    'R' => history.push((history.last().unwrap().0 + 1, history.last().unwrap().1)),
                    'L' => history.push((history.last().unwrap().0 - 1, history.last().unwrap().1)),
                    'U' => history.push((history.last().unwrap().0, history.last().unwrap().1 + 1)),
                    'D' => history.push((history.last().unwrap().0, history.last().unwrap().1 - 1)),
                    _ => unreachable!(),
                }
            }
            2 => {
                input! {
                    p: usize,
                }
                if history.len() < p {
                    println!("{} 0", p - history.len() + 1);
                } else {
                    let his_len = history.len();
                    println!("{} {}", history[his_len - p].0, history[his_len - p].1);
                }
            }
            _ => unreachable!(),
        }
    }
}
