use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans = vec![0; n];
    dfs(&s, 0, &(0..n).collect(), &mut ans);

    println!("{}", ans.iter().join("\n"));
}

fn dfs(s: &Vec<Vec<char>>, cur: usize, idx: &Vec<usize>, ans: &mut Vec<usize>) {
    let mut group = vec![vec![]; 26];
    for &i in idx {
        if cur < s[i].len() {
            group[s[i][cur] as usize - 'a' as usize].push(i);
        }
    }
    for &i in idx {
        ans[i] = cur;
    }
    for g in group {
        if g.len() > 1 {
            dfs(s, cur + 1, &g, ans);
        }
    }
}
