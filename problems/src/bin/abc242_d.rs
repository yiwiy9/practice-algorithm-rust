use proconio::{
    input,
    marker::{Chars, Usize1},
};

/// https://atcoder.jp/contests/abc242/tasks/abc242_d
/// https://atcoder.jp/contests/abc242/editorial/3520
/// => t の全探索するしかなくない、、、？
/// => k=0 の時に処理を終了すると、制約上必ず間に合う
fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    let abc = ['A', 'B', 'C'];

    for &(t, k) in &tk {
        println!("{}", f(&abc, &s, t, k));
    }
}

fn move_char(abc: &[char; 3], c: char, cnt: usize) -> char {
    let mut abc_i = abc.iter().position(|&x| x == c).unwrap();
    abc_i = (abc_i + cnt) % 3;
    abc[abc_i]
}

fn f(abc: &[char; 3], s: &[char], t: usize, k: usize) -> char {
    if t == 0 {
        s[k]
    } else if k == 0 {
        move_char(abc, s[0], t)
    } else {
        let add = if k % 2 == 0 { 1 } else { 2 };
        let c = f(abc, s, t - 1, k / 2);
        move_char(abc, c, add)
    }
}
