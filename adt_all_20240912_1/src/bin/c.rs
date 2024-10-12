use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }

    let mut ans = 0;
    for h in 0..24 {
        let mut cur = 0;
        for &(w, x) in &wx {
            let start = (h + x) % 24;
            if (9..18).contains(&start) {
                cur += w;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
