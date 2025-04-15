use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cnt = vec![1; n];
    let mut leaders = vec![];
    for i in 0..n {
        leaders.push((i, i, i));
    }
    let mut dsu = Dsu::new(n);

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    c: Usize1,
                }

                let x_leader = dsu.leader(x);
                let (left, right, color) = leaders[x_leader];

                let mut new_left = left;
                let mut new_right = right;

                cnt[color] -= dsu.size(x_leader);
                cnt[c] += dsu.size(x_leader);

                if left > 0 {
                    let left_leader = dsu.leader(left - 1);
                    let (left_left, _, left_color) = leaders[left_leader];
                    if c == left_color {
                        dsu.merge(left_leader, x_leader);
                        new_left = left_left;
                    }
                }

                if right < n - 1 {
                    let right_leader = dsu.leader(right + 1);
                    let (_, right_right, right_color) = leaders[right_leader];
                    if c == right_color {
                        dsu.merge(right_leader, x_leader);
                        new_right = right_right;
                    }
                }

                leaders[dsu.leader(x)] = (new_left, new_right, c);
            }
            2 => {
                input! {
                    c: Usize1,
                }
                println!("{}", cnt[c]);
            }
            _ => unreachable!(),
        }
    }
}
