use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    println!(
        "{}",
        if (p <= x && x < p + 100) && (q <= y && y < q + 100) {
            "Yes"
        } else {
            "No"
        }
    );
}
