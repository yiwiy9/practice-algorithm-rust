use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(Usize1, char); q],
    }

    let mut set = HashSet::new();
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            set.insert(i);
        }
    }

    for &(x, c) in &xc {
        match c {
            'A' => {
                if x < n - 2 && s[x + 1] == 'B' && s[x + 2] == 'C' {
                    set.insert(x);
                }
                if x > 0 {
                    set.remove(&(x - 1));
                }
                if x > 1 {
                    set.remove(&(x - 2));
                }
            }
            'B' => {
                if x > 0 && s[x - 1] == 'A' && x < n - 1 && s[x + 1] == 'C' {
                    set.insert(x - 1);
                }
                set.remove(&x);
                if x > 1 {
                    set.remove(&(x - 2));
                }
            }
            'C' => {
                if x > 1 && s[x - 2] == 'A' && s[x - 1] == 'B' {
                    set.insert(x - 2);
                }
                set.remove(&x);
                if x > 0 {
                    set.remove(&(x - 1));
                }
            }
            _ => {
                set.remove(&x);
                if x > 0 {
                    set.remove(&(x - 1));
                }
                if x > 1 {
                    set.remove(&(x - 2));
                }
            }
        }
        s[x] = c;
        println!("{}", set.len());
    }
}
