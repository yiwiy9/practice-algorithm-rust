use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n + 1);
    let mut connected = vec![BinaryHeap::new(); n + 1];
    for i in 1..=n {
        connected[i].push(i as i64);
    }

    for _ in 0..q {
        input! {
            op: u8,
        }
        match op {
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }

                let mut new_leader_heap = BinaryHeap::new();
                for _ in 0..10 {
                    if connected[dsu.leader(u)].is_empty() && connected[dsu.leader(v)].is_empty() {
                        break;
                    }

                    if connected[dsu.leader(u)].peek().unwrap_or(&-1)
                        < connected[dsu.leader(v)].peek().unwrap_or(&-1)
                    {
                        let x = connected[dsu.leader(v)].pop().unwrap();
                        new_leader_heap.push(x);
                    } else {
                        let x = connected[dsu.leader(u)].pop().unwrap();
                        new_leader_heap.push(x);
                    }
                }

                dsu.merge(u, v);
                connected[dsu.leader(u)] = new_leader_heap;
            }
            2 => {
                input! {
                    v: usize,
                    k: usize,
                }

                if connected[dsu.leader(v)].len() < k {
                    println!("-1");
                } else {
                    println!("{}", *connected[dsu.leader(v)].iter().nth(k - 1).unwrap());
                }
            }
            _ => unreachable!(),
        }
    }
}
