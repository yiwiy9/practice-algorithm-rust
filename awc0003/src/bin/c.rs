use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut ab: [(usize,usize); n],
    }

    ab.sort_by(|x, y| (y.0 - y.1).cmp(&(x.0 - x.1)));
    let mut ans: usize = 0;
    for &(a, b) in &ab {
        if k > 0 {
            ans += b;
            k -= 1;
        } else {
            ans += a;
        }
    }

    println!("{}", ans);
}
