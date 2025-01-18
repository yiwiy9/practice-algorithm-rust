use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let mut ans = 0;
    for &s_i in &s {
        let mut ok = false;
        for a in 1..=1000 {
            for b in 1..=1000 {
                ok |= 4 * a * b + 3 * a + 3 * b == s_i;
            }
        }
        if !ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
