use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!(
        "{}",
        String::from("atcoder")
            .chars()
            .skip(l - 1)
            .take(r - l + 1)
            .collect::<String>()
    );
}
