use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    }

    let mut ans = 1 << 60;
    for i in 0..=n {
        for j in 0..=n {
            let cnt = 6 * i + 8 * j;
            if n <= cnt {
                ans = ans.min(s * i + m * j);
                break;
            }
            let k = (n - cnt + 11) / 12;
            ans = ans.min(s * i + m * j + l * k);
        }
    }

    println!("{}", ans);
}
