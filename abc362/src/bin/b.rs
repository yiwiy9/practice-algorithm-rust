use proconio::input;

fn main() {
    input! {
        x_a: i64,
        y_a: i64,
        x_b: i64,
        y_b: i64,
        x_c: i64,
        y_c: i64,
    }

    let ab = (x_b - x_a) * (x_b - x_a) + (y_b - y_a) * (y_b - y_a);
    let bc = (x_c - x_b) * (x_c - x_b) + (y_c - y_b) * (y_c - y_b);
    let ca = (x_a - x_c) * (x_a - x_c) + (y_a - y_c) * (y_a - y_c);

    println!(
        "{}",
        if ab + bc == ca || bc + ca == ab || ca + ab == bc {
            "Yes"
        } else {
            "No"
        }
    );
}
