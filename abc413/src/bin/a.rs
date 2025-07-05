use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        if a.iter().sum::<usize>() <= m {
            "Yes"
        } else {
            "No"
        }
    );
}
