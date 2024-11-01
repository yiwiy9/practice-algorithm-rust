use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n],
    }

    let mut min_r = vec![m + 1; m + 1];
    for (l, r) in lr {
        min_r[l] = min_r[l].min(r);
    }

    let mut heap = std::collections::BinaryHeap::new();
    for l in 1..=m {
        if min_r[l] != m + 1 {
            heap.push((l, min_r[l]));
        }
    }

    let mut ans = 0;
    let mut cur_r = m + 1;
    for l in (1..=m).rev() {
        if !heap.is_empty() {
            let (l_i, r_i) = heap.peek().unwrap();
            if l_i == &l {
                cur_r = cur_r.min(*r_i);
                heap.pop();
            }
        }
        ans += cur_r - l;
    }

    println!("{}", ans);
}
