use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    println!(
        "{}",
        s.iter()
            .map(|&c| c.to_string().repeat(2))
            .collect::<String>()
    );
}
