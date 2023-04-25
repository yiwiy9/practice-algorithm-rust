use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: i32 = 1 << 30;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![true; n]; n];
    for (x, y) in xy {
        g[x][y] = false;
        g[y][x] = false;
    }

    let mut ans = INF;
    for perm in (0_usize..n).permutations(n) {
        let mut sum = 0;
        let mut before_i = n;
        for (j, &i) in perm.iter().enumerate() {
            if j != 0 && !g[before_i][i] {
                sum = -1;
                break;
            }
            sum += a[i][j];
            before_i = i;
        }
        if sum != -1 {
            ans = ans.min(sum);
        }
    }

    println!("{}", if ans != INF { ans } else { -1 });
}
