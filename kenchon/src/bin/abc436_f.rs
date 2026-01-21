use ac_library::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
    }

    let mut ft = FenwickTree::new(n + 1, 0);
    let mut ans: usize = 0;
    for &b_i in &b {
        let left_cnt = ft.sum(0..b_i);
        let right_cnt = b_i - left_cnt;
        ans += (left_cnt + 1) * right_cnt;

        ft.add(b_i, 1);
    }

    println!("{}", ans);
}
