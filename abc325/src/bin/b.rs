use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }

    let mut ans = 0;
    for t in 0..24 {
        let mut cur = 0;
        for &(w, x) in &wx {
            let start = (t + x) % 24;
            if 9 <= start && start < 18 {
                cur += w;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
