use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let max_h = h.iter().max().unwrap();

    println!(
        "{}",
        h.iter()
            .enumerate()
            .find(|&(i, &h_i)| h_i == *max_h)
            .unwrap()
            .0
            + 1
    );
}
