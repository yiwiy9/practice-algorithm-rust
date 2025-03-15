use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }

    let move_count = |from: usize, to: usize, invalid: usize| {
        let mut from = from;
        let mut to = to;
        if from > to {
            std::mem::swap(&mut from, &mut to);
        }

        if from < invalid && invalid < to {
            from + n - to
        } else {
            to - from
        }
    };

    let mut l_pos = 0;
    let mut r_pos = 1;
    let mut ans = 0;
    for &(h, t) in &ht {
        if h == 'L' {
            ans += move_count(l_pos, t, r_pos);
            l_pos = t;
        } else {
            ans += move_count(r_pos, t, l_pos);
            r_pos = t;
        }
    }

    println!("{}", ans);
}
