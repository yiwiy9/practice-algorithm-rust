use ac_library::{Additive, Segtree};
use proconio::{input, marker::Chars};

const MAX: usize = 5 * 100_005;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cum: Vec<i64> = vec![0_i64; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i]
            + match s[i] {
                'A' => 1,
                'B' => -1,
                _ => 0,
            }
    }

    let mut base = 0;
    let cum_min = *cum.iter().min().unwrap();
    if cum_min < 0 {
        base = -cum_min;
        cum = cum.iter().map(|&v| (v + base)).collect();
    }

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(MAX);
    for &cum_i in &cum {
        cnt_segtree.set(cum_i as usize, cnt_segtree.get(cum_i as usize) + 1);
    }

    let mut base_u = base as usize;
    let mut ans: usize = 0;
    for i in 0..n {
        ans += cnt_segtree.prod(base_u + 1..);
        if s[i] == 'A' {
            base_u += 1;
        }
        if s[i] == 'B' {
            base_u -= 1;
        }
        cnt_segtree.set(cum[i] as usize, cnt_segtree.get(cum[i] as usize) - 1);
    }

    println!("{}", ans);
}
