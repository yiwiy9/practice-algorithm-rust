use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }

    let mut f = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                f.push((i, j));
            }
        }
    }

    let n = f.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut cnt = 0;
            for k in 0..n {
                let i_dist = dist(f[i], f[k]);
                let j_dist = dist(f[j], f[k]);
                if i_dist <= d || j_dist <= d {
                    cnt += 1;
                }
            }
            ans = ans.max(cnt);
        }
    }

    println!("{}", ans);
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;
    ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as usize
}
