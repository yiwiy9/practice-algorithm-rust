use itertools::Itertools as _;
use proconio::{input, marker::Chars};

const SNUKE: [char; 5] = ['s', 'n', 'u', 'k', 'e'];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans: Vec<(_, _)> = vec![];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != SNUKE[0] {
                continue;
            }

            let mut right_snuke = vec![(i, j)];
            let mut left_snuke = vec![(i, j)];
            let mut down_snuke = vec![(i, j)];
            let mut up_snuke = vec![(i, j)];
            let mut right_down_snuke = vec![(i, j)];
            let mut left_up_snuke = vec![(i, j)];
            let mut right_up_snuke = vec![(i, j)];
            let mut left_down_snuke = vec![(i, j)];
            for k in 1..5 {
                if j + k < w && s[i][j + k] == SNUKE[k] {
                    right_snuke.push((i, j + k));
                }
                if j >= k && s[i][j - k] == SNUKE[k] {
                    left_snuke.push((i, j - k));
                }
                if i + k < h && s[i + k][j] == SNUKE[k] {
                    down_snuke.push((i + k, j));
                }
                if i >= k && s[i - k][j] == SNUKE[k] {
                    up_snuke.push((i - k, j));
                }
                if i + k < h && j + k < w && s[i + k][j + k] == SNUKE[k] {
                    right_down_snuke.push((i + k, j + k));
                }
                if i >= k && j >= k && s[i - k][j - k] == SNUKE[k] {
                    left_up_snuke.push((i - k, j - k));
                }
                if i >= k && j + k < w && s[i - k][j + k] == SNUKE[k] {
                    right_up_snuke.push((i - k, j + k));
                }
                if i + k < h && j >= k && s[i + k][j - k] == SNUKE[k] {
                    left_down_snuke.push((i + k, j - k));
                }
            }

            if right_snuke.len() == 5 {
                ans = right_snuke;
                break;
            }
            if left_snuke.len() == 5 {
                ans = left_snuke;
                break;
            }
            if down_snuke.len() == 5 {
                ans = down_snuke;
                break;
            }
            if up_snuke.len() == 5 {
                ans = up_snuke;
                break;
            }
            if right_down_snuke.len() == 5 {
                ans = right_down_snuke;
                break;
            }
            if left_up_snuke.len() == 5 {
                ans = left_up_snuke;
                break;
            }
            if right_up_snuke.len() == 5 {
                ans = right_up_snuke;
                break;
            }
            if left_down_snuke.len() == 5 {
                ans = left_down_snuke;
                break;
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|(i, j)| format!("{} {}", i + 1, j + 1))
            .join("\n")
    );
}
