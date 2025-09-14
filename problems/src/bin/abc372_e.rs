use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut leaders_map = HashMap::new();
    for i in 0..n {
        leaders_map.insert(i, vec![i + 1]);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                if dsu.same(u, v) {
                    continue;
                }

                let u_leader = dsu.leader(u);
                let v_leader = dsu.leader(v);

                let mut new_leader_component = vec![];
                let mut u_i = 0;
                let mut v_i = 0;
                for _ in 0..10 {
                    let mut u_num = 0;
                    let mut v_num = 0;
                    if u_i < leaders_map[&u_leader].len() {
                        u_num = leaders_map[&u_leader][u_i];
                    }
                    if v_i < leaders_map[&v_leader].len() {
                        v_num = leaders_map[&v_leader][v_i];
                    }

                    if u_num == 0 && v_num == 0 {
                        break;
                    } else if u_num > v_num {
                        new_leader_component.push(u_num);
                        u_i += 1;
                    } else {
                        new_leader_component.push(v_num);
                        v_i += 1;
                    }
                }

                dsu.merge(u, v);
                let new_leader = dsu.leader(u);
                leaders_map.insert(new_leader, new_leader_component);
            }
            2 => {
                input! {
                    x: Usize1,
                    k: Usize1,
                }

                let leader = dsu.leader(x);
                let component = &leaders_map[&leader];
                if component.len() <= k {
                    println!("-1");
                } else {
                    println!("{}", component[k]);
                }
            }
            _ => unreachable!(),
        }
    }
}
