use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: Usize1,
        mut y: Usize1,
        s: [Chars; h],
        t: Chars,
    }

    let mut set = std::collections::HashSet::new();
    for &c in &t {
        match c {
            'U' => {
                if x > 0 && s[x - 1][y] != '#' {
                    x -= 1;
                }
            }
            'D' => {
                if x + 1 < h && s[x + 1][y] != '#' {
                    x += 1;
                }
            }
            'L' => {
                if y > 0 && s[x][y - 1] != '#' {
                    y -= 1;
                }
            }
            'R' => {
                if y + 1 < w && s[x][y + 1] != '#' {
                    y += 1;
                }
            }
            _ => unreachable!(),
        };

        if s[x][y] == '@' {
            set.insert((x, y));
        }
    }

    println!("{} {} {}", x + 1, y + 1, set.len());
}
