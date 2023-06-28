use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut t = vec![];
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_j) in s.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut s_new = vec![];
            for &c in s_i.iter().rev() {
                s_new.push(c);
            }
            for &c in s_j.iter().rev() {
                s_new.push(c);
            }
            t.push(s_new);
        }
    }

    for t_i in &t {
        let mut ok = true;
        let len = t_i.len();
        for i in 0..len {
            if t_i[i] != t_i[len - 1 - i] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
