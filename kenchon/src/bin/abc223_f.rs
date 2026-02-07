use ac_library::{Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

struct Parenthesis;
impl Monoid for Parenthesis {
    type S = (i64, i64);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        let (l_l_need_cnt, l_r_need_cnt) = *l;
        let (r_l_need_cnt, r_r_need_cnt) = *r;

        let mut next_l_need_cnt = l_l_need_cnt;
        let mut next_r_need_cnt = r_r_need_cnt;

        if l_r_need_cnt <= r_l_need_cnt {
            next_l_need_cnt += r_l_need_cnt - l_r_need_cnt;
        } else {
            next_r_need_cnt += l_r_need_cnt - r_l_need_cnt;
        }

        (next_l_need_cnt, next_r_need_cnt)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(u8, Usize1, Usize1); q],
    }

    let mut parenthesis_segtree = Segtree::<Parenthesis>::new(n);
    for (i, &c) in s.iter().enumerate() {
        parenthesis_segtree.set(i, if c == '(' { (0, 1) } else { (1, 0) });
    }

    for &(op, l, r) in &queries {
        match op {
            1 => {
                let l_val = parenthesis_segtree.get(l);
                let r_val = parenthesis_segtree.get(r);
                parenthesis_segtree.set(l, r_val);
                parenthesis_segtree.set(r, l_val);
            }
            2 => {
                let (l_cnt, r_cnt) = parenthesis_segtree.prod(l..=r);
                println!(
                    "{}",
                    if l_cnt == 0 && r_cnt == 0 {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
