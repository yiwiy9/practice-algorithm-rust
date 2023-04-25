use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ok: bool = true;
    let mut ans: i64 = 0;
    for i in 0..h {
        for j in 0..w {
            a[i][j] -= b[i][j];
            if i != h - 1 && j != w - 1 {
                let diff = -a[i][j];
                a[i][j] += diff;
                a[i + 1][j] += diff;
                a[i][j + 1] += diff;
                a[i + 1][j + 1] += diff;
                ans += diff.abs();
            }
            ok &= a[i][j] == 0;
        }
    }

    if ok {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
