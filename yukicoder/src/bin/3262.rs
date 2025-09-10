use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        lr: [(Usize1, Usize1); n],
    }

    let mut order = (0..n).collect::<Vec<usize>>();
    let mut ans = 0;

    // Heap's algorithmによる順列生成
    heap_recursive(&mut order, n, &lr, &mut ans);

    println!("{}", ans);
}

// 手動で実装した順列生成 (Heap's algorithm) next_permutation
fn heap_recursive(order: &mut Vec<usize>, k: usize, lr: &[(usize, usize)], ans: &mut i32) {
    if k == 1 {
        // 順列をチェック
        let mut ok = true;
        let mut max_left = lr[order[0]].0;
        for i in 1..order.len() {
            let (l, r) = lr[order[i]];
            if max_left > r {
                ok = false;
                break;
            }
            max_left = max_left.max(l);
        }
        if ok {
            *ans += 1;
        }
        return;
    }

    for i in 0..k {
        heap_recursive(order, k - 1, lr, ans);

        // Heap's algorithm: kが偶数なら最初の要素と交換、奇数ならi番目と交換
        if k % 2 == 0 {
            order.swap(i, k - 1);
        } else {
            order.swap(0, k - 1);
        }
    }
}
