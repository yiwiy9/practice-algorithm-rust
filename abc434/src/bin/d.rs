use proconio::{input, marker::Usize1};

const SIZE: usize = 2005;

fn main() {
    input! {
        n: usize,
        udlr: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    let mut clouds = vec![vec![0; SIZE]; SIZE];
    for &(u, d, l, r) in &udlr {
        clouds[u][l] += 1;
        clouds[u][r + 1] -= 1;
        clouds[d + 1][l] -= 1;
        clouds[d + 1][r + 1] += 1;
    }
    for i in 0..SIZE {
        for j in 1..SIZE {
            clouds[i][j] += clouds[i][j - 1];
        }
    }
    for i in 1..SIZE {
        for j in 0..SIZE {
            clouds[i][j] += clouds[i - 1][j];
        }
    }

    // 二次元累積和
    // https://qiita.com/drken/items/56a6b68edef8fc605821#4-%E4%BA%8C%E6%AC%A1%E5%85%83%E7%B4%AF%E7%A9%8D%E5%92%8C
    let mut s = vec![vec![0; SIZE + 1]; SIZE + 1];
    for (i, row) in clouds.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let cur = if c == 1 { 1 } else { 0 };
            s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + cur;
        }
    }

    let mut sum = 2000 * 2000;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if clouds[i][j] == 0 {
                continue;
            }
            sum -= 1;
        }
    }

    for &(u, d, l, r) in &udlr {
        let cur = s[d + 1][r + 1] - s[u][r + 1] - s[d + 1][l] + s[u][l];
        println!("{}", sum + cur);
    }
}
