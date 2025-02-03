use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        f: [[usize;10]; n],
        p: [[i64;11]; n],
    }

    let mut ans = -INF;
    for bit in 1..1 << 10 {
        let mut cur = 0;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..10 {
                if bit & (1 << j) != 0 && f[i][j] == 1 {
                    cnt += 1;
                }
            }
            cur += p[i][cnt];
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
