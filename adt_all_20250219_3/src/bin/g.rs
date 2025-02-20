use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
        t: [Chars; m],
    }

    let mut t_set = HashSet::new();
    for t_i in t {
        t_set.insert(t_i);
    }

    let c_cnt = s.iter().map(|s_i| s_i.iter().len()).sum::<usize>();
    if c_cnt + s.len() - 1 > 16 {
        println!("-1");
        return;
    }
    let available_cnt = 16 + 1 - c_cnt - s.len();

    let mut ok = false;
    // next_permutation()
    permutohedron::heap_recursive(&mut s, |s| {
        if ok {
            return;
        }
        let mut cur = vec![];
        if rec(n, s, &t_set, available_cnt, 0, &mut cur) {
            ok = true;
            println!("{}", cur.iter().collect::<String>());
        }
    });

    if !ok {
        println!("-1");
    }
}

fn rec(
    n: usize,
    s: &[Vec<char>],
    t_set: &HashSet<Vec<char>>,
    available_cnt: usize,
    i: usize,
    cur: &mut Vec<char>,
) -> bool {
    if i == n - 1 {
        cur.extend_from_slice(&s[i]);
        if !t_set.contains(cur) && cur.len() > 2 {
            return true;
        } else {
            cur.truncate(cur.len() - s[i].len());
            return false;
        }
    }

    cur.extend_from_slice(&s[i]);
    cur.push('_');
    for j in 0..=available_cnt {
        cur.extend(std::iter::repeat('_').take(j));
        if rec(n, s, t_set, available_cnt - j, i + 1, cur) {
            return true;
        }
        cur.truncate(cur.len() - j);
    }
    cur.pop();
    cur.truncate(cur.len() - s[i].len());

    false
}
