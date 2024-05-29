use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut history = vec![];
    for i in (1..=n).rev() {
        history.push((i as i32, 0));
    }

    let mut before = (1, 0);
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    dir: char,
                }

                match dir {
                    'R' => before.0 += 1,
                    'L' => before.0 -= 1,
                    'U' => before.1 += 1,
                    'D' => before.1 -= 1,
                    _ => unreachable!(),
                }

                history.push(before);
            }
            2 => {
                input! {
                    p: usize,
                }
                println!(
                    "{} {}",
                    history[history.len() - p].0,
                    history[history.len() - p].1
                );
            }
            _ => unreachable!(),
        }
    }
}
