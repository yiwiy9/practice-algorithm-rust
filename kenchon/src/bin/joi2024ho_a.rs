use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut b = a.iter().map(|&a_i| a_i % t).collect::<Vec<_>>();
    b.sort();

    b.extend_from_slice(&b.iter().map(|b_i| b_i + t).collect::<Vec<_>>());

    let mut ans = INF;
    for i in 0..n {
        ans = ans.min((b[i + n - 1] - b[i]).div_ceil(2));
    }

    println!("{}", ans);
}
