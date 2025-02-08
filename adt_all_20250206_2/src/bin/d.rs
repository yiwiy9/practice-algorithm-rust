use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }

    let move_cnt = |from: usize, to: usize, invalid: usize| {
        let (from, to) = if from < to { (from, to) } else { (to, from) };
        if from < invalid && invalid < to {
            from + n - to
        } else {
            to - from
        }
    };

    let mut ans = 0;
    let mut l = 0;
    let mut r = 1;
    for &(h, t) in &ht {
        if h == 'L' {
            ans += move_cnt(l, t, r);
            l = t;
        } else {
            ans += move_cnt(r, t, l);
            r = t;
        }
    }

    println!("{}", ans);
}
