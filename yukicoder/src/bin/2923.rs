use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
        xy: [(usize,usize); n],
    }

    let mut removed = vec![false; n];

    let mut heap = std::collections::BinaryHeap::new();
    for i in 0..n {
        heap.push((h[i], i));
    }

    while let Some((h_j, j)) = heap.pop() {
        for i in 0..n {
            if i == j {
                continue;
            }
            if h[i] >= h_j {
                continue;
            }

            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            if (x_i as i64 - x_j as i64) * (x_i as i64 - x_j as i64)
                + (y_i as i64 - y_j as i64) * (y_i as i64 - y_j as i64)
                <= (k * k) as i64
            {
                removed[i] = true;
            }
        }
    }

    println!("{}", removed.iter().filter(|&&r| !r).count());
}
