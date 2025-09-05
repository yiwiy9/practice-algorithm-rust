use ac_library::Dsu;
use proconio::input;
use std::collections::HashMap;

const N: usize = 1 << 20;

fn main() {
    input! {
        q: usize,
        tx: [(usize, i64); q],
    }

    let mut a = vec![-1; N];
    let mut dsu = Dsu::new(N);
    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i, (i, i));
    }

    for (t, x) in tx {
        let h_x = (x as usize) % N;
        match t {
            1 => {
                let h = if a[h_x] == -1 {
                    h_x
                } else {
                    (map[&dsu.leader(h_x)].1 + 1) % N
                };

                a[h] = x;

                let left = if h == 0 { N - 1 } else { h - 1 };
                if a[left] != -1 {
                    let component_left = map[&dsu.leader(left)].0;
                    dsu.merge(h, left);
                    let leader = dsu.leader(h);
                    map.insert(leader, (component_left, h));
                }

                let right = (h + 1) % N;
                if a[right] != -1 {
                    let component_right = map[&dsu.leader(right)].1;
                    dsu.merge(h, right);
                    let leader = dsu.leader(h);
                    map.insert(leader, (h, component_right));
                }
            }
            2 => {
                println!("{}", a[h_x]);
            }
            _ => unreachable!(),
        }
    }
}
