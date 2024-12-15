use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let chars = vec!['A', 'B', 'C', 'D', 'E'];
    let points = vec![a, b, c, d, e];
    let mut ans = vec![];
    rec(&points, &chars, &mut ans, &mut (0, vec![]), 0);

    ans.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{}", ans.iter().map(|a| a.1.iter().join("")).join("\n"));
}

fn rec(
    points: &Vec<usize>,
    chars: &Vec<char>,
    ans: &mut Vec<(usize, Vec<char>)>,
    cur: &mut (usize, Vec<char>),
    i: usize,
) {
    if i == points.len() {
        if cur.1.len() == 0 {
            return;
        }
        ans.push(cur.clone());
        return;
    }

    cur.0 += points[i];
    cur.1.push(chars[i]);
    rec(points, chars, ans, cur, i + 1);
    cur.0 -= points[i];
    cur.1.pop();

    rec(points, chars, ans, cur, i + 1);
}
