use proconio::input;

fn main() {
    input! {
        _: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    println!(
        "{}",
        if (x < y && x < z && z < y) || x > y && x > z && z > y {
            "Yes"
        } else {
            "No"
        }
    );
}
