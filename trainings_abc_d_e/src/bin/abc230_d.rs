use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        mut lr: [(i64, i64); n],
    }

    lr.sort_by(|a, b| b.1.cmp(&a.1));

    let mut x = -d;
    let mut ans = 0;
    while let Some((l, r)) = lr.pop() {
        if l < x + d {
            continue;
        }
        x = r;
        ans += 1;
    }

    println!("{}", ans);
}
