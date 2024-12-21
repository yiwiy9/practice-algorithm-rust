use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = xy[i];
                let (x2, y2) = xy[j];
                let (x3, y3) = xy[k];
                if (x1 - x2) * (y1 - y3) != (x1 - x3) * (y1 - y2) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
