use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    let mut has_one = false;
    'outer: for x in 0..h {
        for y in 0..w {
            let mut ok = true;
            'inter: for i in 0..h {
                for j in 0..w {
                    if a[(x + i) % h][(y + j) % w] != b[i][j] {
                        ok = false;
                        break 'inter;
                    }
                }
            }
            if ok {
                has_one = true;
                break 'outer;
            }
        }
    }

    println!("{}", if has_one { "Yes" } else { "No" });
}
