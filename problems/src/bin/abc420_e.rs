use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut black_cnt = vec![0; n];
    let mut is_black = vec![false; n];

    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }

                if !dsu.same(u, v) {
                    let u_black_cnt = black_cnt[dsu.leader(u)];
                    let v_black_cnt = black_cnt[dsu.leader(v)];
                    dsu.merge(u, v);
                    black_cnt[dsu.leader(u)] = u_black_cnt + v_black_cnt;
                }
            }
            2 => {
                input! {
                    v: Usize1,
                }

                if is_black[v] {
                    is_black[v] = false;
                    black_cnt[dsu.leader(v)] -= 1;
                } else {
                    is_black[v] = true;
                    black_cnt[dsu.leader(v)] += 1;
                }
            }
            3 => {
                input! {
                    v: Usize1,
                }

                println!(
                    "{}",
                    if black_cnt[dsu.leader(v)] > 0 {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
