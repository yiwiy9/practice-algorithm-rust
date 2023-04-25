use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a:[i64; n],
        mut b:[i64; n],
    }

    a.sort();
    b.sort();

    println!(
        "{}",
        a.iter()
            .zip(b)
            .fold(0, |acc, (a_i, b_i)| acc + (a_i - b_i).abs())
    );
}
