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

                let u_leader_vec = &leaders_map[&u_leader];
                let v_leader_vec = &leaders_map[&v_leader];
                let mut new_leader_vec = vec![];

                let mut acceptable_len = 10;
                let mut u_leader_i = 0;
                let mut v_leader_i = 0;
                while acceptable_len > 0 {
                    let u_value = u_leader_vec.get(u_leader_i).copied().unwrap_or(0);
                    let v_value = v_leader_vec.get(v_leader_i).copied().unwrap_or(0);
                    if u_value == 0 && v_value == 0 {
                        break;
                    } else if u_value > v_value {
                        new_leader_vec.push(u_value);
                        u_leader_i += 1;
                    } else {
                        new_leader_vec.push(v_value);
                        v_leader_i += 1;
                    }
                    acceptable_len -= 1;
                }

                dsu.merge(u, v);
                let new_leader = dsu.leader(u);
                leaders_map.insert(new_leader, new_leader_vec);
            }
            2 => {
                input! {
                    v: Usize1,
                    k: usize,
                }
                let v_leader = dsu.leader(v);
                println!(
                    "{}",
                    if leaders_map[&v_leader].len() < k {
                        -1
                    } else {
                        leaders_map[&v_leader][k - 1] as i32
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
