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
        t_set.insert(t_i.clone());
    }

    let reserve_cnt = 16 - s.iter().map(|s_i| s_i.len()).sum::<usize>() - (s.len() - 1);

    let mut ok = false;
    // next_permutation()
    permutohedron::heap_recursive(&mut s, |s| {
        if ok {
            return;
        }
        let ordered_s = s.to_vec();
        let mut ans: Vec<char> = Vec::new();
        if rec(&ordered_s, &t_set, &mut ans, reserve_cnt, 0) {
            println!("{}", ans.iter().collect::<String>());
            ok = true
        }
    });

    if !ok {
        println!("-1");
    }
}

fn rec(
    s: &Vec<Vec<char>>,
    t_set: &HashSet<Vec<char>>,
    ans: &mut Vec<char>,
    reserve_cnt: usize,
    i: usize,
) -> bool {
    if i == s.len() - 1 {
        ans.extend(&s[i]);
        if !t_set.contains(ans) && ans.len() >= 3 {
            return true;
        } else {
            ans.truncate(ans.len() - s[i].len());
            return false;
        }
    }

    ans.extend(&s[i]);

    let ans_len = ans.len();
    let mut use_cnt = 0;
    while use_cnt <= reserve_cnt {
        ans.extend(vec!['_'; 1 + use_cnt]);
        if rec(s, t_set, ans, reserve_cnt - use_cnt, i + 1) {
            return true;
        }
        ans.truncate(ans_len);
        use_cnt += 1;
    }

    false
}
