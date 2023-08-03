use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(Usize1, i32); m],
    }

    let mut ok = true;
    let mut ans = vec![-1; n];
    for &(s, c) in &sc {
        if ans[s] == -1 {
            ans[s] = c;
        } else if ans[s] != c {
            ok = false;
            break;
        }
    }

    if !ok || (ans.len() > 1 && ans[0] == 0) {
        println!("{}", -1);
        return;
    }

    println!(
        "{}",
        ans.iter()
            .enumerate()
            .map(|(i, &c)| if c == -1 {
                if i == 0 && ans.len() > 1 {
                    1
                } else {
                    0
                }
            } else {
                c
            })
            .join("")
    );
}
