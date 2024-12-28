use proconio::input;

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
    }

    let ab = (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1);
    let bc = (b.0 - c.0) * (b.0 - c.0) + (b.1 - c.1) * (b.1 - c.1);
    let ca = (c.0 - a.0) * (c.0 - a.0) + (c.1 - a.1) * (c.1 - a.1);

    println!(
        "{}",
        if ab == bc + ca || bc == ca + ab || ca == ab + bc {
            "Yes"
        } else {
            "No"
        }
    );
}
