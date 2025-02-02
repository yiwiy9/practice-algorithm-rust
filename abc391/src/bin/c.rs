use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut pos = (0..n).collect::<Vec<_>>();
    let mut cnt = vec![1; n];
    let mut set = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    p: Usize1,
                    h: Usize1,
                }
                cnt[pos[p]] -= 1;
                if cnt[pos[p]] <= 1 {
                    set.remove(&pos[p]);
                }

                pos[p] = h;
                cnt[pos[p]] += 1;
                if cnt[pos[p]] > 1 {
                    set.insert(pos[p]);
                }
            }
            2 => {
                println!("{}", set.len());
            }
            _ => unreachable!(),
        }
    }
}
