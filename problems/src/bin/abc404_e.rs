use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        c: [usize; n-1],
        a: [usize; n-1],
    }

    let mut max_heap = BinaryHeap::new();
    for (i, &a_i) in a.iter().enumerate() {
        if a_i == 1 {
            max_heap.push(i + 1);
        }
    }
    max_heap.push(0);

    let mut ans = 0;
    let mut cur = max_heap.pop().unwrap();
    while cur > 0 {
        ans += 1;

        let target = *max_heap.peek().unwrap();
        if target + c[cur - 1] >= cur {
            cur = max_heap.pop().unwrap();
            continue;
        }

        let mut next_cur = cur;
        let mut longest = 0;
        for diff in 1..=c[cur - 1] {
            let dist = diff + c[cur - 1 - diff];
            if dist > longest {
                next_cur = cur - diff;
                longest = dist;
            }
        }
        cur = next_cur;
    }

    println!("{}", ans);
}
