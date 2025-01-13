use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize,usize); n],
    }

    for k in 1..=d {
        let mut ans = 0;
        for &(t, l) in tl.iter() {
            ans = ans.max(t * (l + k));
        }
        println!("{}", ans);
    }
}
