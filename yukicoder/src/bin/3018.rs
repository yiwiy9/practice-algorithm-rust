use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

/// https://yukicoder.me/problems/no/3018
/// https://yukicoder.me/problems/no/3018/editorial
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        h: i32,
        w: i32,
    }

    if h == 1 && w == 1 {
        println!("! 1 1");
        return;
    }

    println!("? {} {}", 1, 1);
    stdout().flush().unwrap();

    input! {
        from &mut source,
        d: i32,
    }

    let d_root = (d as f64).sqrt() as i32;

    if h == 1 {
        println!("! 1 {}", d_root + 1);
        return;
    }
    if w == 1 {
        println!("! {} 1", d_root + 1);
        return;
    }

    println!("? {} {}", 1, w);
    stdout().flush().unwrap();

    input! {
        from &mut source,
        d_h: i32,
    }

    for i in 1..=h {
        for j in 1..=w {
            if (i - 1) * (i - 1) + (j - 1) * (j - 1) == d
                && (i - 1) * (i - 1) + (j - w) * (j - w) == d_h
            {
                println!("! {} {}", i, j);
                return;
            }
        }
    }
}
