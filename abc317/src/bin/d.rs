use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }

    let mut all_seats = 0;
    let mut t_seats = 0;
    let mut left_area = vec![];

    for &(x, y, z) in &xyz {
        all_seats += z;
        if x > y {
            t_seats += z;
        } else {
            left_area.push(((y - x) / 2 + 1, z));
        }
    }

    if t_seats > all_seats / 2 {
        println!("{}", 0);
        return;
    }

    let need_seats = all_seats / 2 + 1 - t_seats;

    let m = left_area.len();
    let mut dp = vec![vec![INF; need_seats + 1]; m + 1];
    dp[0][0] = 0;
    for i in 0..m {
        for j in 0..=need_seats {
            if dp[i][j] == INF {
                continue;
            }
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            let new_j = (j + left_area[i].1).min(need_seats);
            dp[i + 1][new_j] = dp[i + 1][new_j].min(dp[i][j] + left_area[i].0);
        }
    }

    println!("{}", dp[m][need_seats]);
}
