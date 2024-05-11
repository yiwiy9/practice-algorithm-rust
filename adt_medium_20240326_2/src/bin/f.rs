use proconio::input;

fn main() {
    input! {
        n: i32,
        q: usize,
    }

    let mut history = vec![];
    for p in (1..=n).rev() {
        history.push((p, 0));
    }

    let mut cur = (1, 0);
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    c: char,
                }
                if c == 'R' {
                    cur.0 += 1;
                } else if c == 'L' {
                    cur.0 -= 1;
                } else if c == 'U' {
                    cur.1 += 1;
                } else if c == 'D' {
                    cur.1 -= 1;
                }
                history.push(cur);
            }
            2 => {
                input! {
                    p: usize,
                }

                let pos = history[history.len() - p];
                println!("{} {}", pos.0, pos.1);
            }
            _ => unreachable!(),
        }
    }
}
