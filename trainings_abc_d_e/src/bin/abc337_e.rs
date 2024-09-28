use itertools::Itertools;
use proconio::{input, marker::Chars, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

/**
 * https://atcoder.jp/contests/abc337/tasks/abc337_e
 * https://atcoder.jp/contests/abc337/editorial/9140
 * 2^k で分割していくと実装がバグるので、発想を変える
 * => 2分探索ではなく、2進数で考える
 */
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        mut n: usize,
    }

    let mut max_bit = 0;
    while (1 << max_bit) < n {
        max_bit += 1;
    }

    let mut a = vec![];
    for i in 0..max_bit {
        let mut a_i = vec![];
        for j in 0..n {
            if j & (1 << i) != 0 {
                a_i.push(j + 1);
            }
        }
        a.push(a_i);
    }

    println!("{}", a.len());
    stdout().flush().unwrap();
    for a_i in &a {
        println!("{} {}", a_i.len(), a_i.iter().join(" "));
        stdout().flush().unwrap();
    }

    input! {
        from &mut source,
        s: Chars,
    }

    let mut ans = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            ans += 1 << i;
        }
    }

    println!("{}", ans + 1);
}
