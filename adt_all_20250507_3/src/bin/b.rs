use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut ans = 0;
    if a >= 4 || b >= 4 {
        ans += 4;
        a = if a >= 4 { a - 4 } else { a };
        b = if b >= 4 { b - 4 } else { b };
    }
    if a >= 2 || b >= 2 {
        ans += 2;
        a = if a >= 2 { a - 2 } else { a };
        b = if b >= 2 { b - 2 } else { b };
    }
    if a >= 1 || b >= 1 {
        ans += 1;
        a = if a >= 1 { a - 1 } else { a };
        b = if b >= 1 { b - 1 } else { b };
    }

    println!("{}", ans);
}
