use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize,usize); n],
    }

    let mut ans = pq.iter().fold(0, |acc, &(p, q)| acc + p * q);
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}
