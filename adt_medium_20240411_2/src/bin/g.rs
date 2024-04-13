use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = INF;
    for i in 1..=n {
        let x = (m + i - 1) / i;
        if x <= n {
            ans = ans.min(i * x);
        }
        if i > x {
            break;
        }
    }

    println!("{}", if ans == INF { -1 } else { ans as i64 });
}
