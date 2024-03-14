use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    println!("{}", rec(x, y, n, 1, true));
}

fn rec(x: usize, y: usize, n: usize, cnt: usize, is_red: bool) -> usize {
    if n == 1 {
        if is_red {
            return 0;
        } else {
            return cnt;
        }
    }

    let mut sum = 0;
    if is_red {
        sum += rec(x, y, n - 1, cnt, true);
        sum += rec(x, y, n, cnt * x, false);
    } else {
        sum += rec(x, y, n - 1, cnt, true);
        sum += rec(x, y, n - 1, cnt * y, false);
    }
    sum
}
