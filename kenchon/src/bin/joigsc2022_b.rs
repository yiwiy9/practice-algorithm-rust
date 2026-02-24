use proconio::input;
use superslice::*;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        ac: [(i64, char) ; n],
        q: usize,
        st: [(i64, i64) ; q],
    }

    let mut a_per_c = vec![vec![]; 4];
    for &(a, c) in &ac {
        let i = match c {
            'J' => 0,
            'O' => 1,
            'I' => 2,
            'G' => 3,
            _ => unreachable!(),
        };
        a_per_c[i].push(a);
    }
    a_per_c.iter_mut().for_each(|v| v.sort());

    for &(s, t) in &st {
        println!("{}", rec(&a_per_c, t, 0, s));
    }
}

fn rec(a_per_c: &[Vec<i64>], t: i64, i: usize, cur: i64) -> i64 {
    if i == 4 {
        return (t - cur).abs();
    }

    let mut res = INF;

    let right_nearest_i = a_per_c[i].lower_bound(&cur);
    if right_nearest_i < a_per_c[i].len() {
        let right_nearest = a_per_c[i][right_nearest_i];
        res = res.min(right_nearest - cur + rec(a_per_c, t, i + 1, right_nearest));
    }

    if right_nearest_i > 0 {
        let left_nearest = a_per_c[i][right_nearest_i - 1];
        res = res.min(cur - left_nearest + rec(a_per_c, t, i + 1, left_nearest));
    }

    res
}
