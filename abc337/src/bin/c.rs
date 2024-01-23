use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut cur = 0;
    let mut graph = vec![n; n];
    for (i, &a_i) in a.iter().enumerate() {
        if a_i == -1 {
            cur = i;
        } else {
            graph[(a_i - 1) as usize] = i;
        }
    }

    let mut ans = vec![];
    loop {
        if cur == n {
            break;
        }
        ans.push(cur + 1);
        cur = graph[cur];
    }

    println!("{}", ans.iter().join(" "))
}
