use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s.iter()
            .enumerate()
            .rev()
            .find(|&(_, &c)| c == 'a')
            .map_or(-1, |(i, _)| i as i32 + 1)
    );
}
