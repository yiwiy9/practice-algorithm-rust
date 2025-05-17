use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut i = 0;
    let mut ii = 0;
    let mut j = 0;
    let mut jj = 0;
    for (k, row) in s.iter().enumerate() {
        for (l, &c) in row.iter().enumerate() {
            if i == 0 && c == '#' {
                i = k + 1;
                j = l + 1;
            }
            if k + 1 == i && c == '#' {
                jj = l + 1;
            }
            if l + 1 == j && c == '#' {
                ii = k + 1;
            }
        }
    }

    println!("{} {}", i, ii);
    println!("{} {}", j, jj);
}
