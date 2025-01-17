use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut ans = 0;
    while a != b {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        if a % b == 0 {
            ans += a / b - 1;
            break;
        }
        ans += a / b;
        a -= (a / b) * b;
    }

    println!("{}", ans);
}
