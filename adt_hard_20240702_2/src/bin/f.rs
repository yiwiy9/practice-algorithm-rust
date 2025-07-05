use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    println!("{}", rec(n, x, y, true));
}

fn rec(n: usize, x: usize, y: usize, is_red: bool) -> usize {
    if n == 1 {
        return if is_red { 0 } else { 1 };
    }

    let mut ans = 0;
    if is_red {
        ans += rec(n - 1, x, y, true);
        ans += x * rec(n, x, y, false);
    } else {
        ans += rec(n - 1, x, y, true);
        ans += y * rec(n - 1, x, y, false);
    }

    ans
}
