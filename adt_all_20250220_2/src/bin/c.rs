use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![]; n];
    for i in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; c_i],
        }
        a[i] = a_i;
    }

    input! {
        x: usize,
    }

    let mut ans = vec![];
    let mut min_cnt = 1_000_000;
    for i in 0..n {
        if a[i].contains(&x) {
            ans.push((a[i].len(), i + 1));
            min_cnt = min_cnt.min(a[i].len());
        }
    }

    let ans_b = ans
        .iter()
        .filter(|&&(cnt, _)| cnt == min_cnt)
        .map(|(_, i)| i)
        .collect_vec();

    println!("{}", ans_b.len());
    println!("{}", ans_b.iter().join(" "));
}
