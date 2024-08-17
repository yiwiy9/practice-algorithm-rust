use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut ans = vec![];
    rec(n, k, 0, 0, &r, &mut vec![], &mut ans);

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}

fn rec(
    n: usize,
    k: usize,
    i: usize,
    sum: usize,
    r: &Vec<usize>,
    cur: &mut Vec<usize>,
    ans: &mut Vec<Vec<usize>>,
) {
    if i == n {
        if sum % k == 0 {
            ans.push(cur.clone());
        }
        return;
    }

    for j in 1..=r[i] {
        cur.push(j);
        rec(n, k, i + 1, sum + j, r, cur, ans);
        cur.pop();
    }
}
