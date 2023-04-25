use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(Usize1, Usize1, i64); q],
    }

    let mut inconvenience = vec![0; n];
    for i in 1..n {
        inconvenience[i] = a[i] - a[i - 1];
    }
    let mut ans = inconvenience.iter().fold(0, |acc, x| acc + x.abs());

    for (l, r, v) in queries {
        if l > 0 {
            ans -= inconvenience[l].abs();
            inconvenience[l] += v;
            ans += inconvenience[l].abs();
        }
        if r + 1 < n {
            ans -= inconvenience[r + 1].abs();
            inconvenience[r + 1] -= v;
            ans += inconvenience[r + 1].abs();
        }
        println!("{}", ans);
    }
}
