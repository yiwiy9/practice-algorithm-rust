use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s.iter()
            .map(|&c| if c == '0' { '1' } else { '0' })
            .collect::<String>()
    );
}
