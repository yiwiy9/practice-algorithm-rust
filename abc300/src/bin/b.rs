use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            let mut ok = true;

            'outer: for k in 0..h {
                for l in 0..w {
                    if a[(i + k) % h][(j + l) % w] != b[k][l] {
                        ok = false;
                        break 'outer;
                    }
                }
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
