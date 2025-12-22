use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }

    let mut ans: usize = 0;
    for row in &a {
        let mut cur = 0;
        for c in row {
            if b.contains(c) {
                cur += 1;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
