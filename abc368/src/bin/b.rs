use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut ans = 0;
    loop {
        if a.iter().filter(|&&x| x > 0).count() <= 1 {
            break;
        }
        a.sort_by(|x, y| y.cmp(x));
        a[0] -= 1;
        a[1] -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
