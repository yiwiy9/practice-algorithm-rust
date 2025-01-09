use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }

    println!(
        "{}",
        match c.as_str() {
            "Red" => g.min(b),
            "Green" => r.min(b),
            "Blue" => r.min(g),
            _ => unreachable!(),
        }
    );
}
