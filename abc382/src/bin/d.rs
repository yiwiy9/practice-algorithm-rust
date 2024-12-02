use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut arr = vec![0; n];
    let mut ans = vec![];

    rec(n, m, 0, &mut arr, &mut ans);

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a.iter().join(" "));
    }
}

fn rec(n: usize, m: usize, i: usize, cur: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
    if i == n {
        ans.push(cur.clone());
        return;
    }

    let start_num = if i == 0 { 1 } else { cur[i - 1] + 10 };
    // 適切に枝刈りをしないと TLE する
    for j in start_num..=(m - 10 * (n - i - 1)) {
        cur[i] = j;
        rec(n, m, i + 1, cur, ans);
    }
}
