use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    println!("{}", rec(x, y, n, true));
}

fn rec(x: usize, y: usize, n: usize, is_red: bool) -> usize {
    if n == 1 {
        if is_red {
            return 0;
        } else {
            return 1;
        }
    }

    let mut res = 0;
    if is_red {
        res += rec(x, y, n - 1, true);
        res += x * rec(x, y, n, false);
    } else {
        res += rec(x, y, n - 1, true);
        res += y * rec(x, y, n - 1, false);
    }

    res
}
