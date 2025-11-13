use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; k],
        l: [usize; q],
    }

    let mut field = vec![false; n];
    for &a_i in &a {
        field[a_i] = true;
    }

    for &l_i in &l {
        let mut cnt = 0;
        for i in 0..n {
            if field[i] {
                cnt += 1;
            }

            if cnt == l_i {
                if i < n - 1 && !field[i + 1] {
                    field[i] = false;
                    field[i + 1] = true;
                }
                break;
            }
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        if field[i] {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.iter().join(" "));
}
