use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
    }

    let mut ans = 0;

    loop {
        if a > b {
            let mut d = a / b;
            if a % b == 0 {
                d -= 1;
            }
            ans += d;
            a -= b * d;
        } else if b > a {
            let mut d = b / a;
            if b % a == 0 {
                d -= 1;
            }
            ans += d;
            b -= a * d;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
