use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut ok_x_left = n;
    let mut ok_y_left = n;
    let mut equal_x_i = n;
    let mut equal_y_i = n;
    for (i, &a_i) in a.iter().enumerate() {
        if ok_x_left == n && a_i <= x {
            ok_x_left = i;
        }
        if ok_y_left == n && a_i >= y {
            ok_y_left = i;
        }
        if a_i == x {
            equal_x_i = i;
        }
        if a_i == y {
            equal_y_i = i;
        }

        if a_i <= x && a_i >= y {
            ans += calc(n, ok_x_left, ok_y_left, equal_x_i, equal_y_i);
        } else {
            ok_x_left = n;
            equal_x_i = n;
            ok_y_left = n;
            equal_y_i = n;
        }
    }

    println!("{}", ans);
}

fn calc(n: usize, ok_x_left: usize, ok_y_left: usize, equal_x_i: usize, equal_y_i: usize) -> usize {
    if equal_x_i == n || equal_y_i == n {
        return 0;
    }

    let outer_left = std::cmp::min(ok_x_left, ok_y_left);
    let inner_left = std::cmp::min(equal_x_i, equal_y_i);

    inner_left - outer_left + 1
}
