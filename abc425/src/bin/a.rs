use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut cur = 1;
        for j in 1..=i {
            cur *= -1;
        }
        cur *= (i as isize) * (i as isize) * (i as isize);
        ans += cur;
    }

    println!("{}", ans);
}
