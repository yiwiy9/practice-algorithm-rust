use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut num_first_i = vec![vec![]; n + 1];
    for (i, &a_i) in a.iter().enumerate().rev() {
        num_first_i[a_i].push(i);
    }

    let mut set = HashSet::new();
    let mut b = vec![0; n];
    for (i, &a_i) in a.iter().enumerate() {
        set.insert(a_i);
        b[i] = set.len();
    }

    let mut b_sum = b.iter().sum::<usize>();

    let mut ans = 0;
    for i in 0..n {
        ans += b_sum;

        num_first_i[a[i]].pop();
        if num_first_i[a[i]].is_empty() {
            b_sum -= n - i
        } else {
            b_sum -= num_first_i[a[i]].last().unwrap() - i
        }
    }

    println!("{}", ans);
}
