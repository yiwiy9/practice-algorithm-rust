use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let m = s.len();
    let mut x = vec![];
    loop {
        let mut ok = false;
        for j in 0..m {
            let s_j = s[j] as usize - 'a' as usize;
            let t_j = t[j] as usize - 'a' as usize;
            if s_j > t_j {
                s[j] = t[j];
                x.push(s.clone());
                ok = true;
                break;
            }
        }

        if ok {
            continue;
        }

        for j in (0..m).rev() {
            let s_j = s[j] as usize - 'a' as usize;
            let t_j = t[j] as usize - 'a' as usize;
            if s_j < t_j {
                s[j] = t[j];
                x.push(s.clone());
                ok = true;
                break;
            }
        }

        if !ok {
            break;
        }
    }

    println!("{}", x.len());
    if !x.is_empty() {
        println!("{}", x.iter().map(|x_i| x_i.iter().join("")).join("\n"));
    }
}
