use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut leader_colors = (0..n).collect::<Vec<_>>();
    let mut color_cnt = vec![1; n];

    // 連結リストをHashMapで簡易的に実現する
    let mut prev_leader_map = HashMap::new();
    let mut next_leader_map = HashMap::new();
    for i in 0..n {
        if i > 0 {
            prev_leader_map.insert(i, i - 1);
        }
        if i < n - 1 {
            next_leader_map.insert(i, i + 1);
        }
    }

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    x: Usize1,
                    c: Usize1,
                }
                let mut leader = dsu.leader(x);
                let cur_color = leader_colors[leader];
                let cur_size = dsu.size(x);

                leader_colors[leader] = c;
                color_cnt[cur_color] -= cur_size;
                color_cnt[c] += cur_size;

                if let Some(&prev_leader) = prev_leader_map.get(&leader) {
                    if leader_colors[prev_leader] == c {
                        dsu.merge(leader, prev_leader);
                        let new_leader = dsu.leader(leader);
                        leader_colors[new_leader] = c;

                        if let Some(&prev_prev_leader) = prev_leader_map.get(&prev_leader) {
                            prev_leader_map.insert(new_leader, prev_prev_leader);
                            next_leader_map.insert(prev_prev_leader, new_leader);
                        }
                        if let Some(&next_leader) = next_leader_map.get(&leader) {
                            next_leader_map.insert(new_leader, next_leader);
                            prev_leader_map.insert(next_leader, new_leader);
                        }

                        leader = new_leader;
                    }
                }

                if let Some(&next_leader) = next_leader_map.get(&leader) {
                    if leader_colors[next_leader] == c {
                        dsu.merge(leader, next_leader);
                        let new_leader = dsu.leader(leader);
                        leader_colors[new_leader] = c;

                        if let Some(&prev_leader) = prev_leader_map.get(&leader) {
                            prev_leader_map.insert(new_leader, prev_leader);
                            next_leader_map.insert(prev_leader, new_leader);
                        }
                        if let Some(&next_next_leader) = next_leader_map.get(&next_leader) {
                            next_leader_map.insert(new_leader, next_next_leader);
                            prev_leader_map.insert(next_next_leader, new_leader);
                        }
                    }
                }
            }
            2 => {
                input! {
                    c: Usize1,
                }
                println!("{}", color_cnt[c]);
            }
            _ => unreachable!(),
        }
    }
}
