use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    let mut left = 0;
    let mut right = 1 << 30;
    while right - left > 1 {
        let mid = (right + left) / 2;

        println!("{}", mid);
        stdout().flush().unwrap();

        input! {
            from &mut source,
            ok: usize,
        }

        if ok == 1 {
            left = mid;
            println!("{}", left);
            return;
        } else {
            right = mid;
        }
    }
}
