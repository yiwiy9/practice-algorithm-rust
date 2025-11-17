use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: i32,
        q: usize,
        abcd: [(Usize1, Usize1, i32, i32); q],
    }

    let mut ans = 0;
    rec(n, m, &abcd, &mut ans, 0, &mut vec![0; n]);

    println!("{}", ans);
}

fn rec(
    n: usize,
    m: i32,
    abcd: &Vec<(usize, usize, i32, i32)>,
    max_points: &mut i32,
    i: usize,
    cur: &mut Vec<i32>,
) {
    if i == n {
        let mut points = 0;
        for &(a, b, c, d) in abcd {
            if cur[b] - cur[a] == c {
                points += d;
            }
        }
        *max_points = (*max_points).max(points);
        return;
    }

    for num in 1..=m {
        if i > 0 && cur[i - 1] > num {
            continue;
        }
        cur[i] = num;
        rec(n, m, abcd, max_points, i + 1, cur);
    }
}
