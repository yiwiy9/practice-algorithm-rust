use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut cnt_ft = FenwickTree::new(n, 0);
    let mut ans: usize = 0;
    for (i, &a_i) in a.iter().enumerate() {
        let lower_cnt = cnt_ft.sum(0..a_i);
        ans += i - lower_cnt;
        cnt_ft.add(a_i, 1);
    }

    println!("{}", ans);
}
