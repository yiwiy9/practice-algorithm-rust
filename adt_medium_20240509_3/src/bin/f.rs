use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![];
    for i in 1..=m {
        rec(n, m, i, &mut vec![i], &mut ans);
    }

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}

fn rec(n: usize, m: usize, i: usize, cur: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
    if cur.len() == n {
        ans.push(cur.clone());
        return;
    }

    for j in (i + 1)..=m {
        cur.push(j);
        rec(n, m, j, cur, ans);
        cur.pop();
    }
}
