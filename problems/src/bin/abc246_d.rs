use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
    }

    let n_root_3 = (n as f64).powf(1.0 / 3.0) as usize + 1;

    let mut ans = INF;
    let mut b_max = n_root_3;
    for a in 0..=n_root_3 {
        for b in (a..=b_max).rev() {
            let x = a * a * a + a * a * b + a * b * b + b * b * b;
            if x < n {
                b_max = b;
                break;
            }
            ans = ans.min(x);
        }
    }

    println!("{}", ans);
}
