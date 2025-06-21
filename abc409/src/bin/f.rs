use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const N: usize = 3000;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut xy: [(i64, i64); n],
    }

    let mut min_k_heap = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let k = (xy[i].0 - xy[j].0).abs() + (xy[i].1 - xy[j].1).abs();
            min_k_heap.push(Reverse((k, i, j)));
        }
    }

    let mut add_cnt = 0;
    let mut dsu = Dsu::new(N);

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    a: i64,
                    b: i64,
                }
                for i in 0..n + add_cnt {
                    let k = (xy[i].0 - a).abs() + (xy[i].1 - b).abs();
                    min_k_heap.push(Reverse((k, i, n + add_cnt)));
                }
                xy.push((a, b));
                add_cnt += 1;
            }
            2 => {
                let mut ans = -1;
                while let Some(Reverse((k, u, v))) = min_k_heap.pop() {
                    if dsu.same(u, v) {
                        continue;
                    }
                    ans = k;
                    dsu.merge(u, v);

                    while let Some(Reverse((kk, uu, vv))) = min_k_heap.peek() {
                        if *kk != ans {
                            break;
                        }
                        dsu.merge(*uu, *vv);
                        min_k_heap.pop();
                    }

                    break;
                }

                println!("{}", ans);
            }
            3 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                println!("{}", if dsu.same(u, v) { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
