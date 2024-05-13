use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 1;
    let mut cnt = 0;
    for a_i in a {
        if cnt + a_i <= k {
            cnt += a_i;
        } else {
            ans += 1;
            cnt = a_i;
        }
    }

    println!("{}", ans);
}
