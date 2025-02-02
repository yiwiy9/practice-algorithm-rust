use proconio::input;

/// https://atcoder.jp/contests/abc391/tasks/abc391_f
/// https://atcoder.jp/contests/abc391/editorial/12085
///
/// A,B,C をそれぞれ降順ソートしておきます．
/// f(i,j,k) = AiBj + BjCk + CkAi とします．
/// K が K ≤ 5×10^5 と小さいことを利用して，上位 K 個を列挙することを考えます．
///
/// 重要な事実として
/// f(i,j,k) ≥ f(i+1,j,k), f(i,j,k) ≥ f(i,j+1,k), f(i,j,k) ≥ f(i,j,k+1) が成り立ちます．
///
/// よって，大きい方から値を列挙していったときに，
/// 必ず f(i,j,k) が列挙された後に f(i+1,j,k),f(i,j+1,k),f(i,j,k+1) が列挙されることが分かります．
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    c.sort_by(|a, b| b.cmp(a));

    let mut heap = std::collections::BinaryHeap::new();
    let mut set = std::collections::HashSet::new();

    let add = |heap: &mut std::collections::BinaryHeap<(usize, usize, usize, usize)>,
               set: &mut std::collections::HashSet<(usize, usize, usize)>,
               i: usize,
               j: usize,
               k: usize| {
        if i >= n || j >= n || k >= n {
            return;
        }
        if set.contains(&(i, j, k)) {
            return;
        }
        heap.push((a[i] * b[j] + b[j] * c[k] + c[k] * a[i], i, j, k));
        set.insert((i, j, k));
    };

    add(&mut heap, &mut set, 0, 0, 0);

    let mut ans = 0;
    for _ in 0..k {
        let (f, i, j, k) = heap.pop().unwrap();
        ans = f;
        add(&mut heap, &mut set, i + 1, j, k);
        add(&mut heap, &mut set, i, j + 1, k);
        add(&mut heap, &mut set, i, j, k + 1);
    }

    println!("{}", ans);
}
