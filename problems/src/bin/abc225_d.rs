use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut link_vec = vec![(0, 0); n + 1];

    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                link_vec[x].1 = y;
                link_vec[y].0 = x;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                link_vec[x].1 = 0;
                link_vec[y].0 = 0;
            }
            3 => {
                input! {
                    x: usize,
                }
                let mut front = x;
                loop {
                    if link_vec[front].0 == 0 {
                        break;
                    }
                    front = link_vec[front].0
                }
                let mut cur = front;
                let mut ans = vec![];
                loop {
                    ans.push(cur);
                    if link_vec[cur].1 == 0 {
                        break;
                    }
                    cur = link_vec[cur].1
                }

                print!("{} ", ans.len());
                println!("{}", ans.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
