use ac_library::{Additive, Segtree};
use proconio::input;

const MAX_A: usize = 1_000_005;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(MAX_A);
    for &a_i in &a {
        cnt_segtree.set(a_i, cnt_segtree.get(a_i) + 1);
    }

    let mut ans = 0;

    // スタート位置のループ
    for num in 1..MAX_A {
        let num_cnt = cnt_segtree.get(num);
        if num_cnt == 0 {
            continue;
        }

        cnt_segtree.set(num, 0);
        ans += (num_cnt - 1) * num_cnt / 2;

        let mut num_ans = 0;
        // 間隔のループ（調和級数）
        // 1 + 1/2 + 1/3 + ... + 1/N = O(log N)
        for mul in 1..MAX_A {
            let cur_num = num * mul;
            if cur_num >= MAX_A {
                break;
            }

            let next_num = (num * (mul + 1)).min(MAX_A);
            num_ans += cnt_segtree.prod(cur_num..next_num) * mul;
        }

        ans += num_ans * num_cnt;
    }

    println!("{}", ans);
}
