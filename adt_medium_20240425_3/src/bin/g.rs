use proconio::input;
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        d: i64,
        mut lr: [(i64,i64); n],
    }

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans = 0;
    let mut cur = -INF;
    for (l, r) in lr {
        if cur + d - 1 >= l {
            continue;
        }

        ans += 1;
        cur = r;
    }

    println!("{}", ans);
}
