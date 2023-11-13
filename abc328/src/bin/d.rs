use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut que = std::collections::VecDeque::new();
    let mut continuous = 0;
    for c in s {
        if c as usize - 'A' as usize == continuous {
            continuous += 1;

            if continuous == 3 {
                continuous = que.pop_back().unwrap_or(0);
            }
        } else if c == 'A' {
            que.push_back(continuous);
            continuous = 1;
        } else {
            if continuous != 0 {
                que.push_back(continuous);
            }
            while let Some(u) = que.pop_front() {
                match u {
                    1 => print!("A"),
                    2 => print!("AB"),
                    _ => unreachable!(),
                }
            }
            print!("{}", c);
            continuous = 0;
        }
    }

    if continuous != 0 {
        que.push_back(continuous);
    }
    while let Some(u) = que.pop_front() {
        match u {
            1 => print!("A"),
            2 => print!("AB"),
            _ => unreachable!(),
        }
    }

    println!();
}
