use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        xy: [(Usize1, Usize1); q],
    }

    for &(x, y) in &xy {
        if (x + n - 1) % n == y || (x + n + 1) % n == y {
            println!("{}", 1);
            continue;
        }

        if s[x] == s[y] {
            println!("{}", 1);
            continue;
        }

        if s[(x + n - 1) % n] == s[y] || s[(x + n + 1) % n] == s[y] {
            println!("{}", 2);
            continue;
        }

        if s[(y + n - 1) % n] == s[x] || s[(y + n + 1) % n] == s[x] {
            println!("{}", 2);
            continue;
        }

        println!("{}", 3);
    }
}
