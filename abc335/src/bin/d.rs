use itertools::Itertools;
use proconio::input;

const DIR: [char; 4] = ['R', 'D', 'L', 'U'];

fn main() {
    input! {
        n: usize,
    }

    let mut cur_pos = (0, 0);
    let mut cur_dir = 0;
    let mut ans = vec![vec![String::from("T"); n]; n];
    for i in 1..=(n * n - 1) {
        ans[cur_pos.0][cur_pos.1] = i.to_string();

        match DIR[cur_dir % 4] {
            'R' => {
                if cur_pos.0 + 1 < n && ans[cur_pos.0 + 1][cur_pos.1] == *"T" {
                    cur_pos = (cur_pos.0 + 1, cur_pos.1);
                } else {
                    cur_dir += 1;
                    cur_pos = (cur_pos.0, cur_pos.1 + 1);
                }
            }
            'L' => {
                if cur_pos.0 > 0 && ans[cur_pos.0 - 1][cur_pos.1] == *"T" {
                    cur_pos = (cur_pos.0 - 1, cur_pos.1);
                } else {
                    cur_dir += 1;
                    cur_pos = (cur_pos.0, cur_pos.1 - 1);
                }
            }
            'U' => {
                if cur_pos.1 > 0 && ans[cur_pos.0][cur_pos.1 - 1] == *"T" {
                    cur_pos = (cur_pos.0, cur_pos.1 - 1);
                } else {
                    cur_dir += 1;
                    cur_pos = (cur_pos.0 + 1, cur_pos.1);
                }
            }
            'D' => {
                if cur_pos.1 + 1 < n && ans[cur_pos.0][cur_pos.1 + 1] == *"T" {
                    cur_pos = (cur_pos.0, cur_pos.1 + 1);
                } else {
                    cur_dir += 1;
                    cur_pos = (cur_pos.0 - 1, cur_pos.1);
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}
