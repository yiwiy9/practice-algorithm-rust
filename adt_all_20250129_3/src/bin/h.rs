use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut min_heap = std::collections::BinaryHeap::new();
    for &x in &a {
        min_heap.push(std::cmp::Reverse(x));
    }

    let mut ans = vec![k; k + 1];
    ans[0] = 0;
    for i in 1..=k {
        while ans[i - 1] == min_heap.peek().unwrap().0 {
            min_heap.pop();
        }

        ans[i] = min_heap.pop().unwrap().0;

        for &x in &a {
            min_heap.push(std::cmp::Reverse(x + ans[i]));
        }
    }

    println!("{}", ans[k]);
}
