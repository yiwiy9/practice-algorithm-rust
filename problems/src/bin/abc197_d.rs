use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        n: f64,
        xy_0: (f64, f64),
        xy_n_half: (f64, f64),
    }

    let xy_center = ((xy_0.0 + xy_n_half.0) / 2., (xy_0.1 + xy_n_half.1) / 2.);

    let mut base = xy_0;

    base.0 -= xy_center.0;
    base.1 -= xy_center.1;

    let radian = 2. * PI / n;

    let mut x_ans = base.0 * radian.cos() - base.1 * radian.sin();
    let mut y_ans = base.0 * radian.sin() + base.1 * radian.cos();

    x_ans += xy_center.0;
    y_ans += xy_center.1;

    println!("{:.7} {:.7}", x_ans, y_ans);
}
