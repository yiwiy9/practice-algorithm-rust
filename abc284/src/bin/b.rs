use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        println!(
            "{}",
            a.iter()
                .filter(|&num| num % 2 == 1)
                .collect::<Vec<_>>()
                .len()
        );
    }
}
