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
    for t_i in &t {
        t_set.insert(t_i.clone());
    }

    let min_len = s.iter().fold(0, |acc, s_i| acc + s_i.len()) + n - 1;
    let remains = 16 - min_len;

    let mut ans = vec![];
    // next_permutation()
    permutohedron::heap_recursive(&mut s, |s| {
        let mut cur = s[0].clone();
        if rec(s, &t_set, remains, &mut cur, 1) {
            ans = cur;
        }
    });

    println!(
        "{}",
        if ans.is_empty() {
            "-1".to_string()
        } else {
            ans.iter().collect::<String>()
        }
    );
}

fn rec(
    s: &[Vec<char>],
    t_set: &HashSet<Vec<char>>,
    remains: usize,
    ans: &mut Vec<char>,
    i: usize,
) -> bool {
    if i == s.len() {
        return 3 <= ans.len() && ans.len() <= 16 && !t_set.contains(ans);
    }

    for cnt in 0..=remains {
        ans.extend(['_'].repeat(cnt + 1));
        ans.extend(s[i].clone());
        if rec(s, t_set, remains - cnt, ans, i + 1) {
            return true;
        }
        ans.truncate(ans.len() - (cnt + 1 + s[i].len()));
    }

    false
}
