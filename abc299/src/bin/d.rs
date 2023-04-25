use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

/**
 * インタラクティブ問題の Rust での実装
 * https://atcoder.jp/contests/abc244/editorial/3625
 *
 * - 出力を行うたびに、末尾に改行を入れて標準出力を flush する
 * - --release のときは「入力を一通り読んでから処理を行う」のに対し、そうでないときは一行ずつ読んで処理を行う
 */
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        mut right: usize,
    }

    let mut left = 1;
    while right - left > 1 {
        let mid = (right + left) / 2;

        println!("? {}", mid);
        stdout().flush().unwrap();

        input! {
            from &mut source,
            mut s_mid: usize,
        }

        if s_mid == 0 {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("! {}", left);
}
