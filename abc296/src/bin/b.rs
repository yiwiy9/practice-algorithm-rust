use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:[Chars;8],
    }

    let line = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let mut xy = ('*', 100);
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                xy.0 = line[j];
                xy.1 = 8 - i;
            }
        }
    }

    println!("{}{}", xy.0, xy.1);
}
