use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    }

    let mut ans = 1 << 30;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if i * 6 + j * 8 + k * 12 >= n {
                    ans = ans.min(i * s + j * m + k * l);
                }
            }
        }
    }

    println!("{}", ans);
}
